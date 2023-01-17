//! Simple command line application for reading and merging bookmkar files.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::unwrap_used,
    rustdoc::missing_crate_level_docs
)]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    exit: bool,
    path: Option<Vec<std::path::PathBuf>>,
}

fn main() {
    let args = Cli::parse();

    let mut lines = args
        .path
        .map(|files| {
            files
                .iter()
                .map(|path| {
                    String::from("load ")
                        + match path.to_str() {
                            Some(path_string) => path_string,
                            None => panic!("{path:?} could not be converted to a string"),
                        }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    if args.exit {
        lines.push("exit".into());
    }

    std::process::exit(bookmark_library::run(
        (!lines.is_empty()).then_some(lines.join("\n")),
        vec![
            Box::new(bookmark_import::Import),
            bookmark_export::Export::as_box(),
        ],
    ));
}
