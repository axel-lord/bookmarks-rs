use crate::{
    command::list,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
}

impl Command for List {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let bookmarks = self.bookmarks.borrow();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(self.buffer.bookmark_count()))
            .map_err(|_| {
                CommandErr::Execution(format!(
                    "could not parse {} as a positive integer",
                    &args[0]
                ))
            })?;

        let from = args
            .get(1)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(0isize))
            .map_err(|_| {
                CommandErr::Execution(format!("could not parse {} as an integer", &args[1]))
            })
            .map(|from| list::wrap_if_negative(from, self.buffer.bookmark_count()))??;

        for (index, bookmark) in shared::Buffer::bookmark_iter(&self.buffer.borrow(), &bookmarks)
            .skip(from)
            .take(count)
        {
            print!("{}. {:#}", index, bookmark);
        }

        Ok(())
    }
}
