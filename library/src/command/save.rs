use std::{fs::File, io::BufWriter};

use bookmark_storage::Listed;

use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save<T>
where
    T: Listed,
{
    storage: shared::Storage<T>,
    buffer: shared::Buffer,
}

impl<T> Command for Save<T>
where
    T: Listed,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        let storage = self.storage.borrow();
        bookmark_storage::save(
            &mut BufWriter::new(File::create(&args[0])?),
            self.buffer
                .iter()
                .map(|i| storage.get(i))
                .take_while(Option::is_some)
                .map(Option::unwrap),
        )?;

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct SaveAll {
    categories: shared::Categroies,
    bookmarks: shared::Bookmarks,
}

impl Command for SaveAll {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        let mut writer = BufWriter::new(File::create(&args[0])?);

        bookmark_storage::save(&mut writer, self.categories.borrow().iter())?;

        bookmark_storage::save(&mut writer, self.bookmarks.borrow().iter())?;

        Ok(())
    }
}
