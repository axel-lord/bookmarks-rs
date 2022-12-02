use bookmark_library::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub fn build(bookmarks: shared::BufferStorage<Bookmark>) -> Box<dyn Command> {
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "import onetab should be called with a single argument".into(),
            ));
        }

        let reader = BufReader::new(File::open(&args[0])?);

        let mut bookmarks = bookmarks.write().unwrap();
        let mut added_count = 0usize;
        for line in reader.lines() {
            let line = line?;
            let Some(url_size) = line.find(" | ") else {continue;};
            let desc_start = url_size + " | ".len();

            let url = &line[0..url_size];
            let desc = &line[desc_start..];

            bookmarks
                .storage
                .push(Bookmark::new(url, desc, std::iter::empty::<&str>()));

            added_count += 1;
        }

        bookmarks.buffer.reset();
        println!("added {} bookmarks", added_count);

        Ok(())
    })
}
