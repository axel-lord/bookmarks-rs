use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use bookmark_library::{
    bookmark::Bookmark, command::CommandErr, command_map::CommandMapBuilder, reset::ResetValues,
    shared::BufferStorage,
};

use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Import;

impl bookmark_library::CommandBuilder for Import {
    fn name(&self) -> &'static str {
        "import"
    }
    fn build(
        &mut self,
        BufferStorage {
            storage: bookmarks, ..
        }: BufferStorage<Bookmark>,
        _categories: BufferStorage<bookmark_library::category::Category>,
        _infos: BufferStorage<bookmark_library::info::Info>,
        reset_values: ResetValues,
    ) -> Box<dyn bookmark_library::command::Command> {
        Box::new(
            CommandMapBuilder::new()
                .name("import".into())
                .push("onetab", Some("import a onetab export"), {
                    let bookmarks = bookmarks.clone();
                    Box::new(move |args: &[String]| {
                        if args.len() != 1 {
                            return Err(CommandErr::Usage(
                                "import onetab should be called with a single argument".into(),
                            ));
                        }

                        let reader = BufReader::new(File::open(&args[0])?);

                        let mut bookmarks = bookmarks.borrow_mut();
                        for line in reader.lines() {
                            let line = line?;
                            let Some(url_size) = line.find(" | ") else {continue;};
                            let desc_start = url_size + " | ".len();

                            let url = &line[0..url_size];
                            let desc = &line[desc_start..];

                            bookmarks.push(Bookmark::new(url, desc, std::iter::empty::<&str>()))
                        }

                        reset_values.reset();

                        Ok(())
                    })
                })
                .push("html", Some("import a firefox html export"), {
                    let bookmarks = bookmarks.clone();
                    Box::new(move |args: &[_]| {
                        if args.len() != 1 {
                            return Err(CommandErr::Usage(
                                "import html should be called with one argument".into(),
                            ));
                        }
                        let contents = io::read_to_string(File::open(&args[0])?)?;

                        let document = Html::parse_document(&contents);

                        if !document.errors.is_empty() {
                            println!("Errors encountered parsing document:");
                        }
                        for err in document.errors.iter() {
                            println!("\t{err}");
                        }

                        let a_selector = Selector::parse("a").unwrap();

                        let mut bookmarks = bookmarks.borrow_mut();
                        let mut added_count = 0usize;
                        for element in document.select(&a_selector) {
                            let Some(url) = element.value().attr("href") else {continue;};
                            let desc = element.inner_html();
                            bookmarks.push(Bookmark::new(url, &desc, std::iter::empty::<&str>()));
                            added_count += 1;
                        }

                        println!("added {} bookmarks", added_count);

                        Ok(())
                    })
                })
                .push("json", Some("parse firefox bookmark backup"), {
                    // let bookmarks = bookmarks.clone();
                    Box::new(move |args: &[_]| {
                        if args.len() != 1 {
                            return Err(CommandErr::Usage(
                                "import json should be called with one argument".into(),
                            ));
                        }

                        let reader = BufReader::new(File::open(&args[0])?);
                        let json: serde_json::Value =
                            serde_json::from_reader(reader).map_err(|err| {
                                CommandErr::Execution(format!("failure parsing json: {err}"))
                            })?;

                        let root = json.as_object().ok_or_else(|| {
                            CommandErr::Execution("root of json file was not an object".into())
                        })?;

                        let mut bookmarks = bookmarks.borrow_mut();
                        let mut element_stack = vec![root];
                        while !element_stack.is_empty() {
                            let top = element_stack.pop().unwrap();

                            if let Some(children) = top.get("children") {
                                let Some(children) = children.as_array() else {
                                    println!("children member not an array\n{:#?}", children);
                                    continue;
                                };

                                for child in children {
                                    let Some(child) = child.as_object() else {
                                        println!("child was not an object\n{:#?}", child);
                                        continue;
                                    };
                                    element_stack.push(child);
                                }
                            } else {
                                let description =
                                    top.get("title").and_then(|v| v.as_str()).unwrap_or("");
                                let Some(url) =
                                    top.get("uri").and_then(|v| v.as_str()) else {continue;};

                                bookmarks.push(Bookmark::new(
                                    url,
                                    description,
                                    std::iter::empty::<&str>(),
                                ))
                            }
                        }

                        Ok(())
                    })
                })
                .build(),
        )
    }
}
