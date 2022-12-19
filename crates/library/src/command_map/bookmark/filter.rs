use crate::{bookmark::Bookmark, shared};
use aho_corasick::AhoCorasickBuilder;
use bookmark_command::{Command, CommandErr};

#[derive(Debug, Command)]
pub struct Filter {
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for Filter {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        let filters = args
            .iter()
            .map(|s| {
                AhoCorasickBuilder::new()
                    .auto_configure(&[s])
                    .ascii_case_insensitive(true)
                    .build([s])
            })
            .collect::<Vec<_>>();

        self.bookmarks.write().unwrap().filter_in_place(|bookmark| {
            filters
                .iter()
                .all(|f| f.is_match(bookmark.url()) || f.is_match(bookmark.description()))
        });

        Ok(())
    }
}

#[derive(Debug, Command)]
pub struct FilterInv {
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for FilterInv {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        self.bookmarks
            .write()
            .unwrap()
            .filter_in_place(|bookmark| !args.iter().any(|arg| bookmark.url().contains(arg)));

        Ok(())
    }
}
