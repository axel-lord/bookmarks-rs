use crate::{
    bookmark::Bookmark,
    command::get_bookmark_iter,
    command_map::{Command, CommandErr},
    token,
};
use std::{
    cell::RefCell,
    fs::File,
    io::{prelude::*, BufWriter},
    ops::Range,
    rc::Rc,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for Save {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        let file = File::create(&args[0]).map_err(|err| {
            CommandErr::Execution(format!("could not open {} for reading: {}", &args[0], err))
        })?;

        let mut writer = BufWriter::new(file);

        let write_err =
            |err| CommandErr::Execution(format!("write to {} failed: {}", &args[0], err));

        writeln!(writer, "{}", token::UNSORTED_BEGIN).map_err(write_err)?;

        for (_, bookmark) in get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()) {
            writeln!(writer, "{}", bookmark.to_line()).map_err(write_err)?;
        }

        writeln!(writer, "{}", token::UNSORTED_END).map_err(write_err)?;

        Ok(())
    }
}
