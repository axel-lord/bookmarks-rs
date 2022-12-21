use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    files: Option<Vec<PathBuf>>,
}

fn main() {
    bookmark_gui::run(Cli::parse().files.unwrap_or_default());
}
