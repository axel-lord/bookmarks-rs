

use crate::{
    command::buffer_length,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Count {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
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
