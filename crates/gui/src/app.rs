use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use bookmark_command::CommandErr;
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
    status: Box<str>,
    url_width: ParsedStr<usize>,
    main_content: MainContent,
}

#[derive(Clone, Copy, Debug)]
pub struct AppView<'a> {
    pub bookmarks: &'a container::BufferStorage<Bookmark>,
    pub categories: &'a container::BufferStorage<Category>,
    pub infos: &'a container::BufferStorage<Info>,
    pub desc_width: (usize, &'a str),
    pub filter: (Option<&'a AhoCorasick>, &'a str),
    pub status: &'a str,
    pub shown_bookmarks: (usize, &'a str),
    pub shown_from: (usize, &'a str),
    pub url_width: (usize, &'a str),
    pub main_content: MainContent,
}

fn load_section<T>(
    source: impl Iterator<Item = (usize, Result<String, io::Error>)>,
    dest: &mut container::BufferStorage<T>,
) where
    T: Listed,
{
    match bookmark_storage::load_from(source) {
        Ok(v) => {
            dest.storage.extend(v.into_iter());
        }
        Err(err) => {
            eprintln!("{err}");
        }
    };
}

impl App {
    fn load_file(&mut self, path: std::path::PathBuf) -> &mut Self {
        let file = match fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{err}");
                return self;
            }
        };

        let mut reader = io::BufReader::new(file).lines().enumerate();

        load_section(reader.by_ref(), &mut self.infos.write().unwrap());
        load_section(reader.by_ref(), &mut self.categories.write().unwrap());
        load_section(reader.by_ref(), &mut self.bookmarks.write().unwrap());

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
            status: "started application".into(),
            filter: None,
            filter_str: "".into(),
            shown_bookmarks: 512.into(),
            shown_from: 0.into(),
            url_width: 75.into(),
            desc_width: 50.into(),
            main_content: MainContent::Bookmarks,
        };

        for file in flags {
            app.load_file(file);
        }

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
                let bookmarks = self.bookmarks.read().unwrap();
                match open::that(bookmarks.storage[i].url()) {
                    Ok(()) => {
                        println!("Successfully opened: {}", bookmarks.storage[i].url());
                        self.status =
                            format!("opened bookmark [{}]", bookmarks.storage[i].url()).into();
                    }
                    Err(err) => {
                        eprintln!("Failed to open: {}, {}", bookmarks.storage[i].url(), err)
                    }
                }
            }

            Msg::ApplyCategory(category_ids) => {
                let mut call_chain = |i: usize| -> Result<(), CommandErr> {
                    self.command_map
                        .call("category", &["select".into(), i.to_string()])?
                        .call("category", &["apply".into()])?;

                    self.status = format!(
                        "applied category <{}>",
                        self.categories.read().unwrap().storage[i].name()
                    )
                    .into();
                    Ok(())
                };

                for i in category_ids.iter() {
                    if let Err(err) = call_chain(*i) {
                        println!("{err}");
                        break;
                    }
                }
            }

            Msg::UpdateShownBookmarks(amount) => {
                if let Ok(msg) = self
                    .shown_bookmarks
                    .parse_with_message(amount, "shown bookmarks")
                {
                    self.status = msg;
                }
            }

            Msg::UpdateShownFrom(f) => {
                if let Ok(msg) = self.shown_from.parse_with_message(f, "shown from") {
                    self.status = msg;
                }
            }

            Msg::UpdateUrlWidth(w) => {
                if let Ok(msg) = self.url_width.parse_with_message(w, "url width") {
                    self.status = msg;
                }
            }

            Msg::UpdateDescWidth(w) => {
                if let Ok(msg) = self.desc_width.parse_with_message(w, "desc width") {
                    self.status = msg;
                }
            }

            Msg::Reset => {
                if let Err(err) = self.command_map.call("reset", &[]) {
                    println!("{err}");
                }
                self.status = "reset bookmark filters".into();
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

        view::application_view(AppView {
            bookmarks: &bookmarks,
            categories: &categories,
            infos: &infos,
            desc_width: self.desc_width.as_tuple(),
            filter: (self.filter.as_ref(), &self.filter_str),
            status: &self.status,
            shown_bookmarks: self.shown_bookmarks.as_tuple(),
            shown_from: self.shown_from.as_tuple(),
            url_width: self.url_width.as_tuple(),
            main_content: self.main_content,
        })
    }
}
