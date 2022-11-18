use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Regex {
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
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

        self.bookmark_buffer
            .filter_in_place(&self.bookmarks, |bookmark| re.is_match(bookmark.url()));

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct RegexInv {
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
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

        self.bookmark_buffer
            .filter_in_place(&self.bookmarks, |bookmark| !re.is_match(bookmark.url()));

        Ok(())
    }
}
