use std::{cell::RefCell, ops::Range, rc::Rc};

use super::buffer_length;

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

pub struct Count {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Count {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
}

impl Command for Count {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "count should be used without any arguments".into(),
            ));
        }

        let total = self.bookmarks.borrow().len();
        let in_buffer = buffer_length(&self.buffer.borrow());

        println!("total: {total}, in buffer: {in_buffer}");

        Ok(())
    }
}
