use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::PathBuf,
    rc::Rc,
};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use bookmark_library::{command_map::CommandMap, container, shared, Bookmark, Category, Info};
use bookmark_storage::Listed;
use iced::{executor, Application, Theme};

use crate::{MainContent, Msg, ParsedStr};

mod view;

#[derive(Debug)]
pub struct App {
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    infos: shared::BufferStorage<Info>,
    command_map: CommandMap<'static>,
    desc_width: ParsedStr<usize>,
    filter: Option<AhoCorasick>,
    filter_str: Box<str>,
    shown_bookmarks: ParsedStr<usize>,
    shown_from: ParsedStr<usize>,
    status_msg: RefCell<Box<str>>,
    status_log: RefCell<Vec<Box<str>>>,
    url_width: ParsedStr<usize>,
    main_content: MainContent,
    category_tree: Box<[Box<[usize]>]>,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView<'a> {
    pub bookmarks: &'a container::BufferStorage<Bookmark>,
    pub categories: &'a container::BufferStorage<Category>,
    pub infos: &'a container::BufferStorage<Info>,
    pub desc_width: (usize, &'a str),
    pub filter: (Option<&'a AhoCorasick>, &'a str),
    pub status: &'a str,
    pub status_log: &'a [Box<str>],
    pub shown_bookmarks: (usize, &'a str),
    pub shown_from: (usize, &'a str),
    pub url_width: (usize, &'a str),
    pub main_content: MainContent,
    pub category_tree: &'a [Box<[usize]>],
}

impl App {
    pub fn set_status(&self, msg: impl Into<Box<str>>) {
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
                dest.storage.extend(v.into_iter());
                format!("loaded section [{}]", T::ITEM_NAME)
            }
            Err(err) => {
                format!("failed to load section [{}], {err}", T::ITEM_NAME)
            }
        });
    }
    fn load_file(&mut self, path: std::path::PathBuf) -> &mut Self {
        let file = match fs::File::open(path.clone()) {
            Ok(file) => file,
            Err(err) => {
                self.set_status(format!("failed to open file \"{}\", {err}", path.display()));
                return self;
            }
        };

        let mut reader = io::BufReader::new(file).lines().enumerate();

        self.load_section(reader.by_ref(), &mut self.infos.write().unwrap());
        self.load_section(reader.by_ref(), &mut self.categories.write().unwrap());
        self.load_section(reader.by_ref(), &mut self.bookmarks.write().unwrap());

        self.set_status(format!("loaded file \"{}\"", path.display()));

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
        )
    }

    fn update_category_tree(&mut self) {
        let categories = self.categories.read().unwrap();
        let infos = self.infos.read().unwrap();

        let cat_map = categories
            .storage
            .iter()
            .enumerate()
            .map(|(i, category)| (<Box<str>>::from(category.id()), i))
            .collect::<HashMap<_, _>>();

        let mut cat_stack = infos
            .storage
            .iter()
            .flat_map(|info| {
                info.categories().rev().map(|id| {
                    (
                        std::iter::empty().collect::<Rc<[usize]>>(),
                        <Box<str>>::from(id),
                    )
                })
            })
            .collect::<Vec<_>>();

        let mut cat_iter = Vec::new();
        while !cat_stack.is_empty() {
            let (depend, cat_id) = cat_stack.pop().unwrap();

            if depend.len() >= 12 {
                continue;
            }

            if let Some(i) = cat_map.get(&cat_id) {
                let category = &categories.storage[*i];
                let this_depend = depend
                    .iter()
                    .cloned()
                    .chain(std::iter::once(*i))
                    .collect::<Vec<usize>>();

                let sub_depend = Rc::<[usize]>::from(this_depend.clone());

                cat_stack.extend(
                    category
                        .subcategories()
                        .rev()
                        .map(|id| (sub_depend.clone(), <Box<str>>::from(id))),
                );

                cat_iter.push(this_depend.into());
            }
        }

        self.category_tree = cat_iter.into();
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
            status_msg: Default::default(),
            status_log: Default::default(),
            filter: None,
            filter_str: "".into(),
            shown_bookmarks: 512.into(),
            shown_from: 0.into(),
            url_width: 75.into(),
            desc_width: 50.into(),
            main_content: MainContent::Bookmarks,
            category_tree: Default::default(),
        };

        app.set_status("Created application");

        for file in flags {
            app.load_file(file);
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
            Msg::GotoBookmarkLocation(i) => {
                self.set_status({
                    let bookmarks = self.bookmarks.read().unwrap();
                    match open::that(bookmarks.storage[i].url()) {
                        Ok(()) => {
                            format!("opened bookmark [{}]", bookmarks.storage[i].url())
                        }
                        Err(err) => {
                            format!("Failed to open: {}, {}", bookmarks.storage[i].url(), err)
                        }
                    }
                });
            }

            Msg::ApplyCategory(category_indices) => {
                let messages = {
                    let categories = self.categories.read().unwrap();
                    let mut bookmarks = self.bookmarks.write().unwrap();

                    category_indices
                        .iter()
                        .cloned()
                        .map(|i| {
                            let category = &categories.storage[i];

                            match category.apply(&mut bookmarks) {
                                Ok(_) => format!("applied category <{}>", category.name()),
                                Err(err) => {
                                    format!(
                                        "failed to apply category <{}>, {}",
                                        category.name(),
                                        err
                                    )
                                }
                            }
                        })
                        .collect::<Vec<_>>()
                };

                for message in messages {
                    self.set_status(message);
                }
            }

            Msg::UpdateShownBookmarks(amount) => {
                if let Ok(msg) = self
                    .shown_bookmarks
                    .parse_with_message(amount, "shown bookmarks")
                {
                    self.set_status(msg);
                }
            }

            Msg::UpdateShownFrom(f) => {
                if let Ok(msg) = self.shown_from.parse_with_message(f, "shown from") {
                    self.set_status(msg);
                }
            }

            Msg::UpdateUrlWidth(w) => {
                if let Ok(msg) = self.url_width.parse_with_message(w, "url width") {
                    self.set_status(msg);
                }
            }

            Msg::UpdateDescWidth(w) => {
                if let Ok(msg) = self.desc_width.parse_with_message(w, "desc width") {
                    self.set_status(msg);
                }
            }

            Msg::Reset => {
                if let Err(err) = self.command_map.call("reset", &[]) {
                    println!("{err}");
                }
                self.set_status("reset bookmark filters");
            }

            Msg::UpdateShownFromSteps(value) => self.shown_from.set_value(Some(
                self.shown_from.value().unwrap_or(0).saturating_add_signed(
                    (self.shown_bookmarks.value().unwrap_or(0) as isize).saturating_mul(value),
                ),
            )),

            Msg::FilterBookmarks(m) => {
                self.filter_str = m;
                self.update_filter();
            }

            Msg::ApplyFilter => {
                if let Some(ref filter) = self.filter {
                    self.bookmarks.write().unwrap().filter_in_place(|b| {
                        filter.is_match(b.url()) || filter.is_match(b.description())
                    });
                }
            }

            Msg::SwitchMainTo(main_content) => self.main_content = main_content,

            Msg::Tick => (),
        }
        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_millis(500)).map(|_| Msg::Tick)
    }

    fn view(&self) -> iced::Element<Msg> {
        let bookmarks = self.bookmarks.read().unwrap();
        let categories = self.categories.read().unwrap();
        let infos = self.infos.read().unwrap();
        let status = self.status_msg.borrow();
        let status_log = self.status_log.borrow();

        view::application_view(AppView {
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
        })
    }
}
