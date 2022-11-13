use std::{cell::RefCell, rc::Rc};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
}

impl Command for Load {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load::load(&args[0])?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no bookmarks parsed from {}",
                &args[0]
            )));
        }

        self.bookmarks.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
