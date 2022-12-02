use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
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
            .buffer
            .write()
            .unwrap()
            .filter_in_place(&self.bookmarks.storage.read().unwrap(), |bookmark| {
                re.is_match(bookmark.url())
            });

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
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
            .buffer
            .write()
            .unwrap()
            .filter_in_place(&self.bookmarks.storage.read().unwrap(), |bookmark| {
                !re.is_match(bookmark.url())
            });

        Ok(())
    }
}
