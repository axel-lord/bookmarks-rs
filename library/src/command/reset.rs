use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    reset,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Reset {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
    selected_bookmark: Rc<RefCell<Option<usize>>>,
}

impl Command for Reset {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "reset should be used without any arguments".into(),
            ));
        }

        reset::reset(
            &mut self.buffer.borrow_mut(),
            &self.bookmarks.borrow(),
            &mut self.selected_bookmark.borrow_mut(),
        );

        Ok(())
    }
}
