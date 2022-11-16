use crate::{command::Command, shared, CommandErr};
use bookmark_storage::{Property, Storeable};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Set {
    bookmarks: shared::Bookmarks,
    selected: shared::Selected,
}

impl Command for Set {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() < 2 {
            return Err(CommandErr::Usage(format!(
                "set needs at least two arguments (a property and a value) {} were given",
                args.len()
            )));
        }

        let mut bookmarks = self.bookmarks.borrow_mut();
        let bookmark = self
            .selected
            .get_mut(&mut bookmarks)
            .ok_or_else(|| CommandErr::Execution("no or an invalid bookmark selected".into()))?;

        bookmark
            .set(&args[0], Property::Single(String::from(&args[1])))
            .or_else(|_| bookmark.set(&args[0], Property::List(Vec::from(&args[1..]))))?;

        print!("{}. {:#}", self.selected.borrow().unwrap(), bookmark);

        Ok(())
    }
}
