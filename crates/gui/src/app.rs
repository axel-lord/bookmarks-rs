use crate::{MainContent, Msg};
use aho_corasick::AhoCorasickBuilder;
use bookmark_library::{command_map::CommandMap, container, shared, Bookmark, Category, Info};
use bookmark_storage::Listed;
use iced::{
    executor,
    widget::{
        self,
        pane_grid::{self, Axis, DragEvent, Pane, ResizeEvent},
    },
    Application, Command, Theme,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    rc::Rc,
    sync::mpsc,
};
use ui::bookmarks_column::BookmarkColumnState;

mod view;

pub use pane::{
    edit::State as EditPaneState, log::State as LogPaneState, Metric, MetricValue, Metrics,
};
pub use view::View;

use self::pane::edit::{BookmarkProxy, CategoryProxy};

pub mod pane;
pub mod ui;

#[derive(Clone, Debug)]
pub enum ChannelMessage {
    GatheredMetric(Metric, MetricValue),
}

/// Application state.
#[derive(Debug)]
pub struct App {
    bookmark_column_state: ui::bookmarks_column::BookmarkColumnState,
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    category_tree: Vec<Vec<usize>>,
    command_map: CommandMap<'static>,
    edit_mode_active: bool,
    infos: shared::BufferStorage<Info>,
    log_panes: pane_grid::State<LogPaneState>,
    edit_panes: pane_grid::State<EditPaneState>,
    settings_pane: Pane,
    main_content: MainContent,
    metrics: Metrics,
    status_log: RefCell<Vec<String>>,
    status_msg: RefCell<String>,
    theme: Theme,
    tick_watcher_count: usize,
    channel: (mpsc::Sender<ChannelMessage>, mpsc::Receiver<ChannelMessage>),
}

impl App {
    /// Create a [View] into the app.
    pub fn to_view<'a>(
        &'a self,
        bookmarks: &'a container::BufferStorage<Bookmark>,
        categories: &'a container::BufferStorage<Category>,
        infos: &'a container::BufferStorage<Info>,
        status: &'a str,
        status_log: &'a [String],
    ) -> View<'a> {
        View {
            bookmarks,
            categories,
            infos,
            desc_width: self.bookmark_column_state.desc_width.as_tuple(),
            filter: (
                self.bookmark_column_state.filter.as_ref(),
                &self.bookmark_column_state.filter_str,
            ),
            status,
            status_log,
            shown_bookmarks: self.bookmark_column_state.shown_bookmarks.as_tuple(),
            shown_from: self.bookmark_column_state.shown_from.as_tuple(),
            url_width: self.bookmark_column_state.url_width.as_tuple(),
            main_content: self.main_content,
            category_tree: &self.category_tree,
            edit_mode_active: self.edit_mode_active,
            bookmark_scrollbar_id: &self.bookmark_column_state.bookmark_scrollbar_id,
            is_dark_mode: matches!(self.theme, Theme::Dark),
            metrics: &self.metrics,
        }
    }

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

    fn edit_bookmark(&mut self, index: usize) {
        let bookmarks = self.bookmarks.read().expect("poisoned lock");
        let bookmark = &bookmarks.storage[index];
        let proxy = BookmarkProxy {
            info: bookmark.description().into(),
            url: bookmark.url().into(),
            tags: bookmark.tags().map(String::from).collect(),
            index,
        };

        let count = self.edit_panes.iter().count();

        self.edit_panes.split(
            if count % 2 == 0 {
                Axis::Horizontal
            } else {
                Axis::Vertical
            },
            &self.settings_pane,
            EditPaneState::Bookmark(proxy),
        );

        self.main_content = MainContent::Edit;
    }

    fn edit_category(&mut self, index: usize) {
        let categories = self.categories.read().expect("posioned lock");
        let category = &categories.storage[index];
        let proxy = CategoryProxy {
            id: category.id().into(),
            name: category.name().into(),
            info: category.description().into(),
            identifiers: category.identifiers().map(String::from).collect(),
            subcategories: category.subcategories().map(String::from).collect(),
            index,
        };

        let count = self.edit_panes.iter().count();

        self.edit_panes.split(
            if count % 2 == 0 {
                Axis::Horizontal
            } else {
                Axis::Vertical
            },
            &self.settings_pane,
            EditPaneState::Category(proxy),
        );

        self.main_content = MainContent::Edit;
    }

    fn recieve_channel_message(&mut self) {
        let Ok(message) = self.channel.1.try_recv() else {
            return;
        };

        match message {
            ChannelMessage::GatheredMetric(metric, value) => {
                if matches!(value, MetricValue::None) {
                    self.set_status(format!("failed to gather metric \"{metric:?}\""));
                } else {
                    self.set_status(format!("gathered metric \"{metric:?}\", value [{value}]"));
                }
                if let MetricValue::UrlMap(ref url_map) = value {
                    let mut pane = *self
                        .log_panes
                        .iter()
                        .next()
                        .expect("there should always be at least one log pane")
                        .0;

                    let pane = loop {
                        match self.log_panes.adjacent(&pane, pane_grid::Direction::Right) {
                            Some(new_pane) => {
                                pane = new_pane;
                            }
                            None => break pane,
                        }
                    };

                    self.log_panes
                        .split(Axis::Vertical, &pane, url_map.clone().into());
                }
                self.metrics.set(metric, value);
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

        let (mut log_panes, log_pane) = pane_grid::State::new(LogPaneState::Log);
        log_panes
            .split(Axis::Vertical, &log_pane, LogPaneState::Stats)
            .expect("splitting log pane should not fail");

        let (edit_panes, settings_pane) = pane_grid::State::new(EditPaneState::Settings);

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
            edit_panes,
            theme: match dark_light::detect() {
                dark_light::Mode::Dark => Theme::Dark,
                dark_light::Mode::Light => Theme::Light,
            },
            metrics: Metrics::default(),
            bookmark_column_state: BookmarkColumnState::default(),
            tick_watcher_count: 0usize,
            channel: mpsc::channel(),
            settings_pane,
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
                        self.increment_tick_watchers(1);
                        Metrics::gather_average_content_string_length(
                            self.channel.0.clone(),
                            self.bookmarks.clone(),
                        );
                    }
                    Metric::UrlOccurances => {
                        self.increment_tick_watchers(1);
                        Metrics::gather_url_occurances(
                            self.channel.0.clone(),
                            self.bookmarks.clone(),
                        );
                    }
                };
                Command::none()
            }
            Msg::CloseLogPane(pane) => {
                self.log_panes.close(&pane);
                Command::none()
            }
            Msg::DragLogPane(drag_event) => {
                if let DragEvent::Dropped { pane, target } = drag_event {
                    self.log_panes.swap(&pane, &target);
                }
                Command::none()
            }
            Msg::Debug(value) => {
                self.set_status(format!("{value:?}"));
                Command::none()
            }
            Msg::CloseEditPane(pane) => {
                self.edit_panes.close(&pane);
                Command::none()
            }
            Msg::DragEditPane(drag_event) => {
                if let DragEvent::Dropped { pane, target } = drag_event {
                    self.edit_panes.swap(&pane, &target);
                }
                Command::none()
            }
            Msg::EditPaneResize(ResizeEvent { ref split, ratio }) => {
                self.edit_panes.resize(split, ratio);
                Command::none()
            }
            Msg::EditBookmarkPaneChange(pane_change) => {
                self.edit_panes
                    .get_mut(&pane_change.pane)
                    .expect("bookmark pane being changed should ecxist")
                    .edit_bookmark(pane_change);
                Command::none()
            }
            Msg::EditCategoryPaneChange(pane_change) => {
                self.edit_panes
                    .get_mut(&pane_change.pane)
                    .expect("bookmark pane being changed should ecxist")
                    .edit_category(pane_change);
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

        ui::view(
            self.to_view(&bookmarks, &categories, &infos, &status, &status_log),
            &self.log_panes,
            &self.edit_panes,
        )
    }
}
