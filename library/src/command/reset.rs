use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    command::{Command, CommandErr},
    reset, shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Reset {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
    selected_bookmark: shared::Selected,
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
