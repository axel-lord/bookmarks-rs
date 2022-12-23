use iced::{
    widget::{radio, Row},
    Alignment, Application, Element,
};
use std::{fmt::Display, path::PathBuf};

mod app;
mod msg;
mod parsed_str;

pub use app::{App, AppView};
pub use msg::Msg;
pub use parsed_str::ParsedStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainContent {
    Bookmarks,
    Settings,
    Log,
}

impl Display for MainContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MainContent {
    pub const MEMBERS: [MainContent; 3] = [
        MainContent::Bookmarks,
        MainContent::Settings,
        MainContent::Log,
    ];

    pub fn choice_row<'a>(&self) -> Element<'a, Msg> {
        Self::MEMBERS
            .iter()
            .map(|mem| {
                radio(format!("{:?}", mem), *mem, Some(*self), Msg::SwitchMainTo)
                    .spacing(3)
                    .size(16)
                    .into()
            })
            .fold(Row::new(), |row, widget: Element<Msg>| row.push(widget))
            .spacing(3)
            .align_items(Alignment::Center)
            .into()
    }
}

pub fn run(starting_files: Vec<PathBuf>) {
    println!("{:?}", MainContent::Bookmarks);
    App::run(iced::Settings {
        flags: starting_files,
        text_multithreading: true,
        ..Default::default()
    })
    .unwrap();
}
