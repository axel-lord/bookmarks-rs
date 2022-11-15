use crate::{
    command::{Command, CommandErr},
    shared,
};
use std::{fs::File, io::BufWriter};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save {
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
}

impl Command for Save {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        bookmark_storage::save(
            &mut BufWriter::new(File::create(&args[0])?),
            shared::Buffer::unenumerated_bookmark_iter(
                &self.buffer.borrow(),
                &self.bookmarks.borrow(),
            ),
        )?;

        Ok(())
    }
}
