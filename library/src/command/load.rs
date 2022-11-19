use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

        let reader = BufReader::new(File::open(&args[0])?);
        let mut lines = reader.lines().enumerate();

        let categories = bookmark_storage::load::load_from(lines.by_ref())?;
        println!("loaded {} categories", categories.len());
        self.categories.borrow_mut().extend(categories.into_iter());

        let bookmarks = bookmark_storage::load::load_from(lines.by_ref())?;
        println!("loaded {} bookmarks", bookmarks.len());
        self.bookmarks.borrow_mut().extend(bookmarks.into_iter());

        self.reset_values.reset();

        Ok(())
    }
}
