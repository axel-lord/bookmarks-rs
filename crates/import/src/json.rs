use bookmark_command::CommandErr;
use bookmark_library::{shared, Bookmark};
use std::{fs::File, io::BufReader};

pub fn build(bookmarks: shared::BufferStorage<Bookmark>) -> Box<dyn bookmark_command::Command> {
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "import json should be called with one argument".into(),
            ));
        }

        let reader = BufReader::new(File::open(&args[0])?);
        let json: serde_json::Value = serde_json::from_reader(reader)
            .map_err(|err| CommandErr::Execution(format!("failure parsing json: {err}")))?;

        let root = json
            .as_object()
            .ok_or_else(|| CommandErr::Execution("root of json file was not an object".into()))?;

        let mut bookmarks = bookmarks.write();
        let mut element_stack = vec![root];
        let mut added_count = 0usize;
        while !element_stack.is_empty() {
            let top = element_stack
                .pop()
                .expect("somehow failed to pop an element from a non-empty stack");

            if let Some(children) = top.get("children") {
                let Some(children) = children.as_array() else {
                    println!("children member not an array\n{children:#?}");
                    continue;
                };

                for child in children {
                    let Some(child) = child.as_object() else {
                        println!("child was not an object\n{child:#?}");
                        continue;
                    };
                    element_stack.push(child);
                }
            } else {
                let description = top
                    .get("title")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("");
                let Some(url) =
                    top.get("uri").and_then(serde_json::Value::as_str) else {continue;};

                bookmarks
                    .storage
                    .push(Bookmark::new(url, description, std::iter::empty::<&str>()));
                added_count += 1;
            }
        }

        println!("added {added_count} bookmarks");
        bookmarks.buffer.reset();

        Ok(())
    })
}
