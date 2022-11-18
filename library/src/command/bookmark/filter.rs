use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Filter {
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
}

impl Command for Filter {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        self.bookmark_buffer
            .filter_in_place(&self.bookmarks, |bookmark| {
                args.iter().all(|arg| bookmark.url().contains(arg))
            });

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct FilterInv {
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
}

impl Command for FilterInv {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        self.bookmark_buffer
            .filter_in_place(&self.bookmarks, |bookmark| {
                !args.iter().any(|arg| bookmark.url().contains(arg))
            });

        Ok(())
    }
}
