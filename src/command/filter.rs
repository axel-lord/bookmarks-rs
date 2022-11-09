use std::{cell::RefCell, ops::Range, rc::Rc};

use super::{get_bookmark_iter, get_filtered_bookmarks};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

pub struct Filter {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Filter {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
}

impl Command for Filter {
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr> {
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

pub struct FilterInv {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl FilterInv {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
}

impl Command for FilterInv {
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr> {
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
