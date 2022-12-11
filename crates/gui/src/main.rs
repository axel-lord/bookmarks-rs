use bookmark_command::CommandErr;
use bookmark_library::{command_map::CommandMap, shared, Bookmark, Category};
use clap::Parser;
use iced::{
    executor,
    widget::{self, button, row, scrollable, text, Column},
    Application, Theme,
};
use std::path::PathBuf;

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
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    BookmarkClicked(usize),
    CategoryClicked(usize),
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
            Msg::BookmarkClicked(i) => {
                let bookmarks = self.bookmarks.read().unwrap();
                match open::that(bookmarks.storage[i].url()) {
                    Ok(()) => println!("Successfully opened: {}", bookmarks.storage[i].url()),
                    Err(err) => {
                        eprintln!("Failed to open: {}, {}", bookmarks.storage[i].url(), err)
                    }
                }
            }

            Msg::CategoryClicked(i) => {
                let call_chain = || -> Result<(), CommandErr> {
                    self.command_map
                        .call("category", &["select".into(), i.to_string()])?
                        .call("category", &["apply".into()])?;
                    Ok(())
                };

                if let Err(err) = call_chain() {
                    println!("{err}");
                }
            }

            Msg::Reset => {
                if let Err(err) = self.command_map.call("reset", &[]) {
                    println!("{err}");
                }
            }
        }
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bookmarks = self.bookmarks.read().unwrap();
        let categories = self.categories.read().unwrap();

        row![
            scrollable(
                categories.iter_indexed().fold(
                    Column::new()
                        .push(text("Categories:"))
                        .push(button("Reset").on_press(Msg::Reset)),
                    |r, (i, c)| {
                        r.push(row![
                            button("Apply").on_press(Msg::CategoryClicked(i)),
                            text(c.name()),
                        ])
                    }
                )
            ),
            scrollable(bookmarks.iter_indexed().take(100).fold(
                Column::new().push(text("Bookmarks:")),
                |r, (i, b)| {
                    r.push(row![
                        button("Goto").on_press(Msg::BookmarkClicked(i)),
                        text(b.description()),
                        widget::Space::new(50.into(), 0.into()),
                        text(b.url())
                    ])
                },
            ))
        ]
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
        },
        ..Default::default()
    })
    .unwrap();
}
