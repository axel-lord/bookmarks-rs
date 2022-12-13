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

#[derive(Debug, Default)]
struct App {
    command_map: CommandMap<'static>,
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
    status: Box<str>,
    shown_bookmarks: Box<str>,
    shown_bookmarks_count: usize,
}

#[derive(Debug, Clone)]
pub enum Msg {
    GotoBookmarkLocation(usize),
    ApplyCategory(usize),
    UpdateStatus(Box<str>),
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
                if let Ok(new_amount) = amount.parse() {
                    self.shown_bookmarks_count = new_amount;
                    self.shown_bookmarks = amount;
                    self.status =
                        format!("changed shown bookmarks to \"{}\"", self.shown_bookmarks).into();
                } else if amount.is_empty() {
                    self.shown_bookmarks_count = 0;
                    self.shown_bookmarks = amount;
                    self.status = "changed shown bookmarks to none".into();
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
            0..self.shown_bookmarks_count,
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
            shown_bookmarks: "100".into(),
            shown_bookmarks_count: 100,
        },
        ..Default::default()
    })
    .unwrap();
}
