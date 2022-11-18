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

        let property = args[0].as_str();

        match bookmark.get(property) {
            Err(err) => return Err(err.into()),
            Ok(Property::List(_)) => {
                bookmark.set(property, Property::List(Vec::from(&args[1..])))?;
            }
            Ok(Property::Single(_)) => {
                if args[1..].len() != 1 {
                    return Err(CommandErr::Execution(format!(
                        "property {} takes only a single value",
                        property
                    )));
                } else {
                    bookmark.set(property, Property::Single(args[1].clone().into()))?;
                }
            }
        }

        println!("{}. {:#}", self.selected.index().unwrap(), bookmark);

        Ok(())
    }
}
