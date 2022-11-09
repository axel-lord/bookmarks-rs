use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

pub struct Reset {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Reset {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
}

impl Command for Reset {
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "reset should be used without any arguments".into(),
            ));
        }

        self.buffer
            .replace(vec![(0..self.bookmarks.borrow().len())]);

        Ok(())
    }
}
