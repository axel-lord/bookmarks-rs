use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc,
    thread,
};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use bookmark_library::{command_map::CommandMap, container, shared, Bookmark, Category, Info};
use bookmark_storage::Listed;
use iced::{
    executor,
    widget::{
        self,
        pane_grid::{self, Axis, ResizeEvent},
    },
    Application, Command, Theme,
};

use crate::{MainContent, Msg};
use conv::prelude::*;

pub mod log_pane;
pub mod view;

pub use log_pane::{LogPane, Metric, MetricValue, Metrics};

use self::view::bookmarks_column::BookmarkColumnState;

#[derive(Clone, Copy, Debug)]
enum ChannelMessage {
    GatheredMetric(Metric, MetricValue),
}

/// Application state.
#[derive(Debug)]
pub struct App {
    bookmark_column_state: view::bookmarks_column::BookmarkColumnState,
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    category_tree: Vec<Vec<usize>>,
    command_map: CommandMap<'static>,
    edit_mode_active: bool,
    infos: shared::BufferStorage<Info>,
    log_panes: pane_grid::State<LogPane>,
    main_content: MainContent,
    metrics: Metrics,
    status_log: RefCell<Vec<String>>,
    status_msg: RefCell<String>,
    theme: Theme,
    tick_watcher_count: usize,
    channel: (mpsc::Sender<ChannelMessage>, mpsc::Receiver<ChannelMessage>),
}

/// View of the application state, providing easy immutable read in building of view.
#[derive(Clone, Copy, Debug)]
pub struct View<'a> {
    /// When bookmarks scrollbar is created will be set to it's id.
    pub bookmark_scrollbar_id: &'a widget::scrollable::Id,
    /// Bookmarks loaded by application.
    pub bookmarks: &'a container::BufferStorage<Bookmark>,
    /// Categories loaded by application.
    pub categories: &'a container::BufferStorage<Category>,
    /// Category indices arranged with parents at start.
    pub category_tree: &'a [Vec<usize>],
    /// Expected max character count of bookmark descriptions deisplayed as numeric and str.
    pub desc_width: (usize, &'a str),
    /// Is true if edit mode is enabled.
    pub edit_mode_active: bool,
    /// Filter used for bookmarks as filter object and str.
    pub filter: (Option<&'a AhoCorasick>, &'a str),
    /// Info loaded by application.
    pub infos: &'a container::BufferStorage<Info>,
    /// True if dark mode in use.
    pub is_dark_mode: bool,
    /// What is expected to fill the main area.
    pub main_content: MainContent,
    /// Stats that may have been gathered.
    pub metrics: &'a Metrics,
    /// How many bookmarks are shown as numeric and str.
    pub shown_bookmarks: (usize, &'a str),
    /// Where in the bookmark list to start showing bookmarks as numeric and str.
    pub shown_from: (usize, &'a str),
    /// Current status message.
    pub status: &'a str,
    /// All status messages.
    pub status_log: &'a [String],
    /// Expected max cahgracter count of bookmark urls displayed as numeric and str.
    pub url_width: (usize, &'a str),
}

impl<'a> View<'a> {
    fn from_app(
        app: &'a App,
        bookmarks: &'a container::BufferStorage<Bookmark>,
        categories: &'a container::BufferStorage<Category>,
        infos: &'a container::BufferStorage<Info>,
        status: &'a str,
        status_log: &'a [String],
    ) -> Self {
        View {
            bookmarks,
            categories,
            infos,
            desc_width: app.bookmark_column_state.desc_width.as_tuple(),
            filter: (
                app.bookmark_column_state.filter.as_ref(),
                &app.bookmark_column_state.filter_str,
            ),
            status,
            status_log,
            shown_bookmarks: app.bookmark_column_state.shown_bookmarks.as_tuple(),
            shown_from: app.bookmark_column_state.shown_from.as_tuple(),
            url_width: app.bookmark_column_state.url_width.as_tuple(),
            main_content: app.main_content,
            category_tree: &app.category_tree,
            edit_mode_active: app.edit_mode_active,
            bookmark_scrollbar_id: &app.bookmark_column_state.bookmark_scrollbar_id,
            is_dark_mode: matches!(app.theme, Theme::Dark),
            metrics: &app.metrics,
        }
    }
}

impl App {
    /// Set the current status and add it to log.
    pub fn set_status(&self, msg: impl Into<String>) {
        let msg = msg.into();
        println!("status: {msg}");
        self.status_log.borrow_mut().push(msg.clone());
        self.status_msg.replace(msg);
    }

    fn load_section<T>(
        &self,
        source: impl Iterator<Item = (usize, Result<String, io::Error>)>,
        dest: &mut container::BufferStorage<T>,
    ) where
        T: Listed,
    {
        self.set_status(match bookmark_storage::load_from(source) {
            Ok(v) => {
                dest.storage.as_mut().reserve(v.len());
                dest.storage.extend(v);
                format!("loaded section [{}]", T::ITEM_NAME)
            }
            Err(err) => {
                format!("failed to load section [{}], {err}", T::ITEM_NAME)
            }
        });
    }

    fn load_file(&mut self, path: &Path) -> &mut Self {
        let file = match fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                self.set_status(format!("failed to open file \"{}\", {err}", path.display()));
                return self;
            }
        };

        let mut reader = io::BufReader::new(file).lines().enumerate();

        macro_rules! load_sections {
            ($($sect:expr),* $(,)?) => {
                $(
                self.load_section(reader.by_ref(), &mut $sect.write().expect("posioned lock"));
                )*
            };
        }

        let before = std::time::Instant::now();
        load_sections!(self.infos, self.categories, self.bookmarks,);
        let duration = std::time::Instant::now().duration_since(before);

        self.set_status(format!(
            "loaded file \"{}\" in {} milliseconds",
            path.display(),
            duration.as_secs_f64() * 1000.0,
        ));

        self
    }

    fn update_filter(&mut self) {
        if self.bookmark_column_state.filter_str.is_empty() {
            self.bookmark_column_state.filter = None;
            return;
        }

        let patterns: &[&str] = &[&self.bookmark_column_state.filter_str];

        self.bookmark_column_state.filter = Some(
            AhoCorasickBuilder::new()
                .auto_configure(patterns)
                .ascii_case_insensitive(true)
                .build(patterns),
        );
    }

    fn update_category_tree(&mut self) {
        let categories = self.categories.read().expect("poisoned lock");
        let infos = self.infos.read().expect("posoned lock");

        let cat_map = categories
            .storage
            .iter()
            .enumerate()
            .map(|(i, category)| (<String>::from(category.id()), i))
            .collect::<HashMap<_, _>>();

        let mut cat_stack = infos
            .storage
            .iter()
            .flat_map(|info| {
                info.categories().rev().map(|id| {
                    (
                        std::iter::empty().collect::<Rc<[usize]>>(),
                        <String>::from(id),
                    )
                })
            })
            .collect::<Vec<_>>();

        let mut cat_iter = Vec::new();
        while let Some((depend, cat_id)) = cat_stack.pop() {
            if depend.len() >= 12 {
                continue;
            }

            if let Some(i) = cat_map.get(&cat_id) {
                let category = &categories.storage[*i];
                let this_depend = depend
                    .iter()
                    .copied()
                    .chain(std::iter::once(*i))
                    .collect::<Vec<usize>>();

                let sub_depend = Rc::<[usize]>::from(this_depend.clone());

                cat_stack.extend(
                    category
                        .subcategories()
                        .rev()
                        .map(|id| (sub_depend.clone(), <String>::from(id))),
                );

                cat_iter.push(this_depend);
            }
        }

        self.category_tree = cat_iter;
    }

    fn apply_category(&mut self, indices: impl IntoIterator<Item = usize>) {
        let messages = {
            let categories = self.categories.read().expect("posoned lock");
            let mut bookmarks = self.bookmarks.write().expect("posoned lock");

            indices
                .into_iter()
                .map(|i| {
                    let category = &categories.storage[i];

                    match category.apply(&mut bookmarks) {
                        Ok(_) => format!("applied category <{}>", category.name()),
                        Err(err) => {
                            format!("failed to apply category <{}>, {}", category.name(), err)
                        }
                    }
                })
                .collect::<Vec<_>>()
        };

        for message in messages {
            self.set_status(message);
        }
    }

    fn goto_bookmark_location(&self, index: usize) {
        self.set_status({
            let bookmarks = self.bookmarks.read().expect("posioned lock");
            match open::that(bookmarks.storage[index].url()) {
                Ok(()) => {
                    format!("opened bookmark [{}]", bookmarks.storage[index].url())
                }
                Err(err) => {
                    format!(
                        "Failed to open: {}, {}",
                        bookmarks.storage[index].url(),
                        err
                    )
                }
            }
        });
    }

    fn edit_bookmark(&mut self, _index: usize) {
        dbg!(self);
        todo!()
    }

    fn edit_category(&mut self, _index: usize) {
        dbg!(self);
        todo!()
    }

    fn recieve_channel_message(&mut self) {
        let Ok(message) = self.channel.1.try_recv() else {
            return;
        };

        match message {
            ChannelMessage::GatheredMetric(metric, value) => {
                self.metrics.set(metric, value);
                if matches!(value, MetricValue::None) {
                    self.set_status(format!("failed to gather metric \"{metric:?}\""));
                } else {
                    self.set_status(format!("gathered metric \"{metric:?}\", value [{value}]"));
                }
                self.decrement_tick_watchers(1);
            }
        }
    }

    fn increment_tick_watchers(&mut self, amount: usize) {
        let old = self.tick_watcher_count;

        self.tick_watcher_count = old.saturating_add(amount);

        self.set_status(format!(
            "incremented tick watcher count from {old} to {}",
            self.tick_watcher_count
        ));
    }

    fn decrement_tick_watchers(&mut self, amount: usize) {
        let old = self.tick_watcher_count;

        self.tick_watcher_count = old.saturating_sub(amount);

        self.set_status(format!(
            "decremented tick watcher count from {old} to {}",
            self.tick_watcher_count
        ));
    }
}

impl Default for App {
    fn default() -> Self {
        let bookmarks = shared::BufferStorage::default();
        let categories = shared::BufferStorage::default();
        let infos = shared::BufferStorage::default();

        let (mut log_panes, log_pane) = pane_grid::State::new(LogPane::Log);
        log_panes.split(Axis::Vertical, &log_pane, LogPane::Stats);

        Self {
            command_map: CommandMap::default_config(
                bookmarks.clone(),
                categories.clone(),
                infos.clone(),
            )
            .build(),
            bookmarks,
            categories,
            infos,
            status_msg: RefCell::default(),
            status_log: RefCell::default(),
            main_content: MainContent::Bookmarks,
            category_tree: Vec::new(),
            edit_mode_active: false,
            log_panes,
            theme: match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
            },
            metrics: Metrics::default(),
            bookmark_column_state: BookmarkColumnState::default(),
            tick_watcher_count: 0usize,
            channel: mpsc::channel(),
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = Vec<PathBuf>;
    type Message = Msg;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Self::default();

        app.set_status("Created application");

        for file in flags {
            app.load_file(&file);
        }

        app.update_category_tree();

        (app, iced::Command::none())
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if self.tick_watcher_count > 0 {
            iced::time::every(std::time::Duration::from_millis(50)).map(|_| Msg::Tick)
        } else {
            iced::Subscription::none()
        }
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        "Application".into()
    }

    #[allow(clippy::too_many_lines)] // due to having to handle a lot of message types, perhaps
                                     // look into dynamic dispatch.
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Msg::None => Command::none(),

            Msg::Tick => {
                self.recieve_channel_message();
                Command::none()
            }

            Msg::GotoBookmarkLocation(i) => {
                self.goto_bookmark_location(i);
                Command::none()
            }

            Msg::ApplyCategory(indices) => {
                self.apply_category(indices);
                Command::none()
            }

            Msg::UpdateShownBookmarks(amount) => {
                if let Ok(msg) = self
                    .bookmark_column_state
                    .shown_bookmarks
                    .parse_with_message(&amount, "shown bookmarks")
                {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateShownFrom(f) => {
                if let Ok(msg) = self
                    .bookmark_column_state
                    .shown_from
                    .parse_with_message(&f, "shown from")
                {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateUrlWidth(w) => {
                if let Ok(msg) = self
                    .bookmark_column_state
                    .url_width
                    .parse_with_message(&w, "url width")
                {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateDescWidth(w) => {
                if let Ok(msg) = self
                    .bookmark_column_state
                    .desc_width
                    .parse_with_message(&w, "desc width")
                {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::Reset => {
                if let Err(err) = self.command_map.call("reset", &[]) {
                    println!("{err}");
                }
                self.set_status("reset bookmark filters");

                Command::none()
            }

            Msg::UpdateShownFromSteps(value) => {
                self.bookmark_column_state.shown_from.set_value(Some(
                    self.bookmark_column_state
                        .shown_from
                        .value()
                        .unwrap_or(0)
                        .saturating_add_signed(
                            isize::try_from(
                                self.bookmark_column_state
                                    .shown_bookmarks
                                    .value()
                                    .unwrap_or(0),
                            )
                            .expect("shown bookmarks too large to convert to isize")
                            .saturating_mul(value),
                        ),
                ));
                widget::scrollable::snap_to(
                    self.bookmark_column_state.bookmark_scrollbar_id.clone(),
                    0.0,
                )
            }

            Msg::FilterBookmarks(m) => {
                self.bookmark_column_state.filter_str = m;
                self.update_filter();

                Command::none()
            }

            Msg::ApplyFilter => {
                if let Some(ref filter) = self.bookmark_column_state.filter {
                    self.bookmarks
                        .write()
                        .expect("poisoned lock")
                        .filter_in_place(|b| {
                            filter.is_match(b.url()) || filter.is_match(b.description())
                        });
                }

                Command::none()
            }

            Msg::SwitchMainTo(main_content) => {
                self.main_content = main_content;
                Command::none()
            }

            Msg::AddBookmarks(bookmarks) => {
                if let Ok(mut bookmarks) = bookmarks.lock() {
                    if let Some(bookmarks) = bookmarks.take() {
                        self.bookmarks
                            .write()
                            .expect("posioned lock")
                            .storage
                            .extend(bookmarks);
                    }
                }

                Command::none()
            }

            Msg::SetEditMode(val) => {
                self.edit_mode_active = val;

                Command::none()
            }

            Msg::EditBookmark(index) => {
                self.edit_bookmark(index);
                Command::none()
            }

            Msg::EditCategory(index) => {
                self.edit_category(index);
                Command::none()
            }
            Msg::LogPaneResize(ResizeEvent { ref split, ratio }) => {
                self.log_panes.resize(split, ratio);
                Command::none()
            }
            Msg::SetTheme(theme) => {
                self.theme = theme;
                Command::none()
            }

            Msg::GatherMetric(metric) => {
                match metric {
                    Metric::AverageContentStringLength => {
                        let bookmarks = self.bookmarks.clone();
                        let tx = self.channel.0.clone();
                        self.increment_tick_watchers(1);

                        thread::spawn(move || {
                            let bookmarks = bookmarks.read().expect("posioned lock");
                            tx.send(ChannelMessage::GatheredMetric(
                                Metric::AverageContentStringLength,
                                (|| {
                                    let sum = f64::value_from(
                                        bookmarks
                                            .storage
                                            .iter()
                                            .map(Bookmark::stored_length)
                                            .sum::<usize>(),
                                    )
                                    .ok()?;

                                    let average =
                                        sum / f64::value_from(bookmarks.storage.len()).ok()?;

                                    Some(average)
                                })()
                                .map_or_else(|| MetricValue::None, MetricValue::Float),
                            ))
                        });
                    }
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Msg> {
        let bookmarks = self.bookmarks.read().expect("poisoned lock");
        let categories = self.categories.read().expect("poisoned lock");
        let infos = self.infos.read().expect("poisoned lock");
        let status = self.status_msg.borrow();
        let status_log = self.status_log.borrow();

        view::view(
            View::from_app(self, &bookmarks, &categories, &infos, &status, &status_log),
            &self.log_panes,
        )
    }
}
