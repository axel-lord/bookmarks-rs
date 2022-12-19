use clap::Parser;
use iced::Application;
use std::path::PathBuf;

mod app;
mod msg;
mod parsed_str;

pub use app::{App, AppView};
pub use msg::Msg;
pub use parsed_str::ParsedStr;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    files: Option<Vec<PathBuf>>,
}

fn main() {
    let args = Cli::parse();

    App::run(iced::Settings {
        flags: args.files.unwrap_or_default(),
        ..Default::default()
    })
    .unwrap();
}
