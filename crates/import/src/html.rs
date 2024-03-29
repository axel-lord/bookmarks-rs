use bookmark_library::{shared, Bookmark};
use scraper::{Html, Selector};
use std::{fs::File, io};

pub fn build(bookmarks: shared::BufferStorage<Bookmark>) -> Box<dyn bookmark_command::Command> {
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(bookmark_command::CommandErr::Usage(
                "import html should be called with one argument".into(),
            ));
        }
        let contents = io::read_to_string(File::open(&args[0])?)?;

        let document = Html::parse_document(&contents);

        if !document.errors.is_empty() {
            println!("Errors encountered parsing document:");
        }
        for err in &document.errors {
            println!("\t{err}");
        }

        let a_selector = Selector::parse("a")
            .expect("could not create a selector for links (should never happen)");

        let mut bookmarks = bookmarks.write();
        let mut added_count = 0usize;
        for element in document.select(&a_selector) {
            let Some(url) = element.value().attr("href") else {continue;};
            let desc = element.inner_html();
            bookmarks
                .storage
                .push(Bookmark::new(url, &desc, std::iter::empty::<&str>()));
            added_count += 1;
        }

        println!("added {added_count} bookmarks");
        bookmarks.buffer.reset();

        Ok(())
    })
}
