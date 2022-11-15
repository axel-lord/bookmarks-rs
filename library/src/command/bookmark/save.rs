use crate::{
    bookmark::Bookmark,
    command::get_bookmark_iter,
    command::{Command, CommandErr},
    shared,
};
use std::{cell::RefCell, fs::File, io::BufWriter, ops::Range, rc::Rc};

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
            get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()).map(|(_, b)| b),
        )?;

        Ok(())
    }
}
