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

fn category_column<'a, Renderer>(
    categories: impl IntoIterator<Item = (usize, impl AsRef<Category>)>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::text::StyleSheet,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::button::StyleSheet,
{
    categories.into_iter().fold(
        Column::new()
            .push(text("Categories:"))
            .push(button("Reset").on_press(Msg::Reset)),
        |r, (i, c)| {
            r.push(row![
                button("Apply").on_press(Msg::CategoryClicked(i)),
                text(c.as_ref().name().to_string()),
            ])
        },
    )
}

fn bookmark_column<'a, Renderer>(
    bookmarks: impl IntoIterator<Item = (usize, impl AsRef<Bookmark>)>,
) -> Column<'a, Msg, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::text::StyleSheet,
    <Renderer as iced_native::Renderer>::Theme: iced::widget::button::StyleSheet,
{
    bookmarks
        .into_iter()
        .take(100)
        .fold(Column::new().push(text("Bookmarks:")), |r, (i, b)| {
            r.push(row![
                button("Goto").on_press(Msg::BookmarkClicked(i)),
                text(b.as_ref().description()),
                widget::Space::new(50.into(), 0.into()),
                text(b.as_ref().url())
            ])
        })
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
            scrollable(category_column(categories.iter_indexed())),
            scrollable(bookmark_column(bookmarks.iter_indexed()))
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
