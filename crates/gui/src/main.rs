use bookmark_command::CommandErr;
use bookmark_library::{command_map::CommandMap, shared, Bookmark, Category};
use clap::Parser;
use iced::{executor, Application, Theme};
use std::path::PathBuf;

mod view;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    files: Option<Vec<PathBuf>>,
}

mod parsed_str {
    use std::{fmt::Display, ops::Deref, str::FromStr};

    #[derive(Debug, Clone)]
    pub struct ParsedStr<V> {
        string: Box<str>,
        val: Option<V>,
    }

    impl<V> Default for ParsedStr<V> {
        fn default() -> Self {
            Self {
                string: "".into(),
                val: None,
            }
        }
    }

    impl<V> AsRef<str> for ParsedStr<V> {
        fn as_ref(&self) -> &str {
            &self.string
        }
    }

    impl<V> Deref for ParsedStr<V> {
        type Target = str;
        fn deref(&self) -> &Self::Target {
            self.as_ref()
        }
    }

    impl<V> From<V> for ParsedStr<V>
    where
        V: ToString,
    {
        fn from(value: V) -> Self {
            Self {
                string: value.to_string().into(),
                val: Some(value),
            }
        }
    }

    impl<V> FromStr for ParsedStr<V>
    where
        V: FromStr,
    {
        type Err = <V as FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.is_empty() {
                Ok(Self {
                    string: "".into(),
                    val: None,
                })
            } else {
                let val = s.parse()?;
                Ok(Self {
                    string: s.into(),
                    val: Some(val),
                })
            }
        }
    }

    impl<V> ParsedStr<V>
    where
        V: ToString,
    {
        pub fn value(&self) -> &Option<V> {
            &self.val
        }

        pub fn set_value(&mut self, val: Option<V>) {
            self.string = val
                .as_ref()
                .map(|v| v.to_string().into())
                .unwrap_or_else(|| "".into());
            self.val = val;
        }
    }

    impl<V> ParsedStr<V>
    where
        V: FromStr + Display,
    {
        pub fn parse_with_message(
            &mut self,
            from: impl Into<Box<str>>,
            msg: &str,
        ) -> Result<Box<str>, <V as FromStr>::Err> {
            let string = from.into();
            let out_msg;

            (self.val, out_msg) = if string.is_empty() {
                (None, format!("changed {msg} to none").into())
            } else {
                let val = string.parse()?;
                let out_msg = format!("changed {msg} to {val}").into();
                (Some(val), out_msg)
            };

            self.string = string;

            Ok(out_msg)
        }
    }
}
pub use parsed_str::ParsedStr;

#[derive(Debug, Default)]
struct App {
    command_map: CommandMap<'static>,
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    status: Box<str>,
    shown_bookmarks: ParsedStr<usize>,
    url_width: ParsedStr<usize>,
    desc_width: ParsedStr<usize>,
}

#[derive(Debug, Clone)]
pub enum Msg {
    GotoBookmarkLocation(usize),
    ApplyCategory(usize),
    UpdateStatus(Box<str>),
    UpdateUrlWidth(Box<str>),
    UpdateDescWidth(Box<str>),
    Reset,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = Self;
    type Message = Msg;
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (flags, iced::Command::none())
    }

    fn title(&self) -> String {
        "Application".into()
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

            Msg::ApplyCategory(i) => {
                let mut call_chain = || -> Result<(), CommandErr> {
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

                if let Err(err) = call_chain() {
                    println!("{err}");
                }
            }

            Msg::UpdateStatus(amount) => {
                if let Ok(msg) = self
                    .shown_bookmarks
                    .parse_with_message(amount, "shown bookmarks")
                {
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
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bookmarks = self.bookmarks.read().unwrap();
        let categories = self.categories.read().unwrap();

        view::application_view(
            &bookmarks,
            &categories,
            &self.status,
            &self.shown_bookmarks,
            0..self.shown_bookmarks.value().unwrap_or(0),
            &self.url_width,
            &self.desc_width,
        )
        .into()
    }
}

fn main() {
    let args = Cli::parse();

    let bookmarks = shared::BufferStorage::default();
    let categories = shared::BufferStorage::default();
    let infos = shared::BufferStorage::default();

    let command_map =
        CommandMap::default_config(bookmarks.clone(), categories.clone(), infos).build();

    if let Some(files) = args.files {
        for file in files {
            command_map
                .call("load", &[file.to_string_lossy().into()])
                .unwrap();
        }
    }

    App::run(iced::Settings {
        flags: App {
            command_map,
            bookmarks,
            categories,
            status: "started application".into(),
            shown_bookmarks: 512.into(),
            url_width: 75.into(),
            desc_width: 50.into(),
        },
        ..Default::default()
    })
    .unwrap();
}
