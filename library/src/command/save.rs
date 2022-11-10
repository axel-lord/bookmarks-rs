use crate::{
    bookmark::Bookmark,
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

use super::get_bookmark_iter;

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

        let Ok(file) = File::create(&args[0]) else {
            return Err(CommandErr::Execution(format!("could not open {} for writing", &args[0])));
        };

        let mut writer = BufWriter::new(file);

        let write_err = |_| CommandErr::Execution(format!("write to {} failed", &args[0]));

        writeln!(writer, "{}", token::UNSORTED_BEGIN).map_err(write_err)?;

        for (_, bookmark) in get_bookmark_iter(&self.bookmarks.borrow(), &self.buffer.borrow()) {
            writeln!(writer, "{}", bookmark.to_line()).map_err(write_err)?;
        }

        writeln!(writer, "{}", token::UNSORTED_END).map_err(write_err)?;

        Ok(())
    }
}
