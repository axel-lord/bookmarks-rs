use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    info::Info,
    reset::ResetValues,
    shared,
};

use bookmark_storage::Listed;

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load<T> {
    buffer_storage: shared::BufferStorage<T>,
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

        self.buffer_storage
            .storage
            .write()
            .unwrap()
            .extend(loaded.into_iter());

        self.reset_values.reset();

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct LoadAll {
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
    infos: shared::BufferStorage<Info>,
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

        let infos = bookmark_storage::load::load_from(lines.by_ref())?;
        println!("loaded {} infos", infos.len());
        self.infos
            .storage
            .write()
            .unwrap()
            .extend(infos.into_iter());

        let categories = bookmark_storage::load::load_from(lines.by_ref())?;
        println!("loaded {} categories", categories.len());
        self.categories
            .storage
            .write()
            .unwrap()
            .extend(categories.into_iter());

        let bookmarks = bookmark_storage::load::load_from(lines.by_ref())?;
        println!("loaded {} bookmarks", bookmarks.len());
        self.bookmarks
            .storage
            .write()
            .unwrap()
            .extend(bookmarks.into_iter());

        self.reset_values.reset();

        Ok(())
    }
}
