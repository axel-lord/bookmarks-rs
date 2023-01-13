//! Crate for graphical interface for manipulating bookmarks.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

use iced::{
    widget::{radio, Row},
    Alignment, Application, Element,
};
use std::{fmt::Display, path::PathBuf};

mod app;
mod msg;
mod parsed_str;

pub use app::{App, View};
pub use msg::Msg;
pub use parsed_str::ParsedStr;

/// Enum representing what content the main area can hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainContent {
    /// Main area holds bookmark list.
    Bookmarks,
    /// Main area holds settings editor.
    Edit,
    /// Main area holds log.
    Log,
}

impl Display for MainContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl MainContent {
    const RADIO_CHOCES: [MainContent; 3] =
        [MainContent::Bookmarks, MainContent::Edit, MainContent::Log];

    /// Generate an area chooser for current area.
    #[must_use]
    pub fn choice_row<'a>(&self) -> Element<'a, Msg> {
        match self {
            MainContent::Bookmarks | MainContent::Edit | MainContent::Log => {
                MainContent::RADIO_CHOCES
                    .iter()
                    .map(|mem| {
                        <Element<Msg>>::from(
                            radio(format!("{mem:?}"), *mem, Some(*self), Msg::SwitchMainTo)
                                .spacing(3)
                                .size(16),
                        )
                    })
                    .fold(Row::new(), Row::push)
                    .spacing(3)
                    .align_items(Alignment::Center)
                    .into()
            }
        }
    }
}

/// Attempt to run the application, loading any file paths passed.
///
/// # Errors
/// If iced fails to start application.
pub fn run(starting_files: Vec<PathBuf>) -> Result<(), iced::Error> {
    println!("{:?}", MainContent::Bookmarks);
    App::run(iced::Settings {
        flags: starting_files,
        text_multithreading: true,
        ..Default::default()
    })
}
