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
                .map(|s| String::from("load ") + s.to_str().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    if args.exit {
        lines.push("exit".into());
    }

    std::process::exit(bookmark_library::run(
        (!lines.is_empty()).then_some(lines.join("\n")),
        vec![Box::new(bookmark_import::Import)],
    ));
}
