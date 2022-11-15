use std::{cell::RefCell, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Select {
    bookmarks: shared::Bookmarks,
    selected: shared::Selected,
}

impl Command for Select {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "select should be called with one argument".into(),
            ));
        }

        let index = args[0].parse().map_err(|_| {
            CommandErr::Usage(format!(
                "could not parse {} as a positive integer",
                &args[0]
            ))
        })?;

        if !(..self.bookmarks.borrow().len()).contains(&index) {
            return Err(CommandErr::Execution(format!(
                "{index} is not the index of a bookmark"
            )));
        }

        self.selected.borrow_mut().replace(index);

        print!(
            "selected bookmark:\n{}. {:#}",
            index,
            self.bookmarks.borrow()[index]
        );

        Ok(())
    }
}
