use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};

use bookmark_command::Command;

#[derive(Debug, Command)]
pub struct Regex {
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for Regex {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution("regex needs a pattern".into()));
        }

        let pattern = args.join(" ");
        let Ok(re) = regex::Regex::new(&pattern) else {
            return Err(CommandErr::Execution(format!("invalid pattern /{pattern}/")));
        };

        self.bookmarks
            .write()
            .unwrap()
            .filter_in_place(|bookmark| re.is_match(bookmark.url()));

        Ok(())
    }
}

#[derive(Debug, Command)]
pub struct RegexInv {
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for RegexInv {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution("regex needs a pattern".into()));
        }

        let pattern = args.join(" ");
        let Ok(re) = regex::Regex::new(&pattern) else {
            return Err(CommandErr::Execution(format!("invalid pattern /{pattern}/")));
        };

        self.bookmarks
            .write()
            .unwrap()
            .filter_in_place(|bookmark| !re.is_match(bookmark.url()));

        Ok(())
    }
}
