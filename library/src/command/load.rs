use std::fs::File;

use crate::{
    command::{Command, CommandErr},
    reset::ResetValues,
    shared,
};

use bookmark_storage::Listed;

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load<T>
where
    T: Listed,
{
    destination: shared::Storage<T>,
    reset_values: ResetValues,
}

impl<T> Command for Load<T>
where
    T: Listed,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no lines parsed from {}",
                &args[0]
            )));
        }

        // self.destination.borrow_mut().extend_from_slice(&loaded);
        let mut destination = self.destination.borrow_mut();
        for loaded in loaded.into_iter() {
            destination.push(loaded);
        }

        self.reset_values.reset();

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct LoadAll {
    categories: shared::Categroies,
    bookmarks: shared::Bookmarks,
    reset_values: ResetValues,
}

impl Command for LoadAll {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no category lines parsed from {}",
                &args[0]
            )));
        }

        self.categories.borrow_mut().extend(loaded.into_iter());

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no bookmark lines parsed from {}",
                &args[0]
            )));
        }

        self.bookmarks.borrow_mut().extend(loaded.into_iter());

        self.reset_values.reset();

        Ok(())
    }
}
