use std::{cell::RefCell, ops::Range, rc::Rc};

use super::{get_bookmark_iter, get_filtered_bookmarks};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

#[derive(Debug)]
pub struct Regex {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Regex {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
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

        let filtered = get_filtered_bookmarks(
            get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()),
            |bookmark| re.is_match(bookmark.url()),
        );

        self.buffer.replace(filtered);

        Ok(())
    }
}

#[derive(Debug)]
pub struct RegexInv {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl RegexInv {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
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

        let filtered = get_filtered_bookmarks(
            get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()),
            |bookmark| !re.is_match(bookmark.url()),
        );

        self.buffer.replace(filtered);

        Ok(())
    }
}
