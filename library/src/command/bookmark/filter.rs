use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command::{get_bookmark_iter, get_filtered_bookmarks},
    command::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Filter {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for Filter {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        let filtered = get_filtered_bookmarks(
            get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()),
            |bookmark| args.iter().all(|arg| bookmark.url().contains(arg)),
        );

        self.buffer.replace(filtered);

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct FilterInv {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for FilterInv {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.is_empty() {
            return Err(CommandErr::Execution(
                "filter needs one or more arguments".into(),
            ));
        }

        let filtered = get_filtered_bookmarks(
            get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()),
            |bookmark| !args.iter().any(|arg| bookmark.url().contains(arg)),
        );

        self.buffer.replace(filtered);

        Ok(())
    }
}
