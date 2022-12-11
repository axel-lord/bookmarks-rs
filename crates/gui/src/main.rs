use bookmark_library::{command_map::CommandMap, shared, Bookmark};
use clap::Parser;
use iced::{
    executor,
    widget::{self, row, text, Column},
    Application, Theme,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    files: Option<Vec<PathBuf>>,
}

#[derive(Debug)]
struct App {
    _command_map: CommandMap<'static>,
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = (CommandMap<'static>, shared::BufferStorage<Bookmark>);
    type Message = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                _command_map: flags.0,
                bookmarks: flags.1,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        "Application".into()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.bookmarks
            .read()
            .unwrap()
            .iter()
            .take(100)
            .fold(Column::new().push(text("Bookmarks:")), |r, b| {
                r.push(row![
                    text(b.description()).width(500.into()),
                    widget::Space::new(50.into(), 0.into()),
                    text(b.url())
                ])
            })
            .into()
    }
}

fn main() {
    let args = Cli::parse();

    let bookmarks = shared::BufferStorage::default();
    let categories = shared::BufferStorage::default();
    let infos = shared::BufferStorage::default();

    let command_map = CommandMap::default_config(bookmarks.clone(), categories, infos).build();

    if let Some(files) = args.files {
        for file in files {
            command_map
                .call("load", &[file.to_string_lossy().into()])
                .unwrap();
        }
    }

    App::run(iced::Settings {
        flags: (command_map, bookmarks),
        ..Default::default()
    })
    .unwrap();
}
