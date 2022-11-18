use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Count {
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
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
            self.bookmarks.len(),
            self.bookmark_buffer.bookmark_count(),
        );

        Ok(())
    }
}
