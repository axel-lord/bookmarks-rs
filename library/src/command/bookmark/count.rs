use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command::buffer_length,
    command::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Count {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for Count {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "count should be used without any arguments".into(),
            ));
        }

        println!(
            "total: {}, in buffer: {}",
            self.bookmarks.borrow().len(),
            buffer_length(&self.buffer.borrow())
        );

        Ok(())
    }
}
