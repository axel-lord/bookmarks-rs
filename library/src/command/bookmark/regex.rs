use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Regex {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
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

        let filtered = self.buffer.filter(&self.bookmarks.borrow(), |bookmark| {
            re.is_match(bookmark.url())
        });

        self.buffer.replace(filtered);

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct RegexInv {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
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

        let filtered = self.buffer.filter(&self.bookmarks.borrow(), |bookmark| {
            !re.is_match(bookmark.url())
        });

        self.buffer.replace(filtered);

        Ok(())
    }
}
