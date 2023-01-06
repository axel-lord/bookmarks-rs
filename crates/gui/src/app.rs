use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
    rc::Rc,
};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use bookmark_library::{command_map::CommandMap, container, shared, Bookmark, Category, Info};
use bookmark_storage::Listed;
use iced::{executor, widget, Application, Command, Theme};

use crate::{MainContent, Msg, ParsedStr};

mod view;

/// Application state.
#[derive(Debug)]
pub struct App {
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    category_tree: Vec<Vec<usize>>,
    command_map: CommandMap<'static>,
    desc_width: ParsedStr<usize>,
    edit_mode_active: bool,
    filter: Option<AhoCorasick>,
    filter_str: String,
    infos: shared::BufferStorage<Info>,
    main_content: MainContent,
    shown_bookmarks: ParsedStr<usize>,
    shown_from: ParsedStr<usize>,
    status_log: RefCell<Vec<String>>,
    status_msg: RefCell<String>,
    url_width: ParsedStr<usize>,
    bookmark_scrollbar_id: widget::scrollable::Id,
}

/// View of the application state, providing easy immutable read in building of view.
#[derive(Clone, Copy, Debug)]
pub struct View<'a> {
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
    /// What is expected to fill the main area.
    pub main_content: MainContent,
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

    /// When bookmarks scrollbar is created will be set to it's id.
    pub bookmark_scrollbar_id: &'a widget::scrollable::Id,
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
        if self.filter_str.is_empty() {
            self.filter = None;
            return;
        }

        let patterns: &[&str] = &[&self.filter_str];

        self.filter = Some(
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
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = Vec<PathBuf>;
    type Message = Msg;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let bookmarks = shared::BufferStorage::default();
        let categories = shared::BufferStorage::default();
        let infos = shared::BufferStorage::default();

        let mut app = Self {
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
            filter: None,
            filter_str: String::new(),
            shown_bookmarks: 512.into(),
            shown_from: 0.into(),
            url_width: 75.into(),
            desc_width: 50.into(),
            main_content: MainContent::Bookmarks,
            category_tree: Vec::new(),
            edit_mode_active: false,
            bookmark_scrollbar_id: widget::scrollable::Id::unique(),
        };

        app.set_status("Created application");

        for file in flags {
            app.load_file(&file);
        }

        app.update_category_tree();

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        "Application".into()
    }

    fn theme(&self) -> Self::Theme {
        match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Msg::Tick | Msg::None => Command::none(),

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
                    .shown_bookmarks
                    .parse_with_message(&amount, "shown bookmarks")
                {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateShownFrom(f) => {
                if let Ok(msg) = self.shown_from.parse_with_message(&f, "shown from") {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateUrlWidth(w) => {
                if let Ok(msg) = self.url_width.parse_with_message(&w, "url width") {
                    self.set_status(msg);
                }

                Command::none()
            }

            Msg::UpdateDescWidth(w) => {
                if let Ok(msg) = self.desc_width.parse_with_message(&w, "desc width") {
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
                self.shown_from.set_value(Some(
                    self.shown_from.value().unwrap_or(0).saturating_add_signed(
                        isize::try_from(self.shown_bookmarks.value().unwrap_or(0))
                            .expect("shown bookmarks too large to convert to isize")
                            .saturating_mul(value),
                    ),
                ));
                widget::scrollable::snap_to(self.bookmark_scrollbar_id.clone(), 0.0)
            }

            Msg::FilterBookmarks(m) => {
                self.filter_str = m;
                self.update_filter();

                Command::none()
            }

            Msg::ApplyFilter => {
                if let Some(ref filter) = self.filter {
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
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| Msg::Tick)
    }

    fn view(&self) -> iced::Element<Msg> {
        let bookmarks = self.bookmarks.read().expect("poisoned lock");
        let categories = self.categories.read().expect("poisoned lock");
        let infos = self.infos.read().expect("poisoned lock");
        let status = self.status_msg.borrow();
        let status_log = self.status_log.borrow();

        view::view(View {
            bookmarks: &bookmarks,
            categories: &categories,
            infos: &infos,
            desc_width: self.desc_width.as_tuple(),
            filter: (self.filter.as_ref(), &self.filter_str),
            status: &status,
            status_log: &status_log,
            shown_bookmarks: self.shown_bookmarks.as_tuple(),
            shown_from: self.shown_from.as_tuple(),
            url_width: self.url_width.as_tuple(),
            main_content: self.main_content,
            category_tree: &self.category_tree,
            edit_mode_active: self.edit_mode_active,
            bookmark_scrollbar_id: &self.bookmark_scrollbar_id,
        })
    }
}
