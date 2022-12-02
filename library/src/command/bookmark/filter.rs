use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
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

        self.bookmarks
            .buffer
            .write()
            .unwrap()
            .filter_in_place(&self.bookmarks.storage.read().unwrap(), |bookmark| {
                args.iter().all(|arg| bookmark.url().contains(arg))
            });

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
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
            .buffer
            .write()
            .unwrap()
            .filter_in_place(&self.bookmarks.storage.read().unwrap(), |bookmark| {
                !args.iter().any(|arg| bookmark.url().contains(arg))
            });

        Ok(())
    }
}
