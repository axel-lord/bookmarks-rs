use iced::Application;
use std::path::PathBuf;

mod app;
mod msg;
mod parsed_str;

pub use app::{App, AppView};
pub use msg::Msg;
pub use parsed_str::ParsedStr;

pub fn run(starting_files: Vec<PathBuf>) {
    App::run(iced::Settings {
        flags: starting_files,
        text_multithreading: true,
        ..Default::default()
    })
    .unwrap();
}
