fn main() {
    let start = std::env::args()
        .map(|s| String::from("load ") + &s)
        .collect::<Vec<_>>()[1..]
        .join("\n");
    std::process::exit(bookmark_library::run(
        if start.is_empty() { None } else { Some(start) },
        vec![Box::new(bookmark_import::OnetabImport)],
    ));
}
