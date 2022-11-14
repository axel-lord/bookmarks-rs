use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command::{buffer_length, get_bookmark_iter, list},
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for List {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let bookmarks = self.bookmarks.borrow();
        let buffer = self.buffer.borrow();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(buffer_length(&buffer)))
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
            .map(|from| list::wrap_if_negative(from, buffer_length(&buffer)))??;

        for (_, bookmark) in get_bookmark_iter(&bookmarks, &buffer)
            .skip(from)
            .take(count)
        {
            println!("{} {}", bookmark.url(), bookmark.description());
        }

        Ok(())
    }
}
