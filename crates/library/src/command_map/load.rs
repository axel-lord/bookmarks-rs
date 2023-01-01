use crate::{bookmark::Bookmark, category::Category, info::Info, shared};
use bookmark_command::{Command, CommandErr};
use bookmark_storage::Listed;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Command)]
pub struct Load<T> {
    buffer_storage: shared::BufferStorage<T>,
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

        let mut buffer_storage = self.buffer_storage.write().expect("poisoned write lock");

        buffer_storage.storage.extend(loaded.into_iter());
        buffer_storage.buffer.reset();

        Ok(())
    }
}

#[derive(Debug, Command)]
pub struct LoadAll {
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
    infos: shared::BufferStorage<Info>,
}

macro_rules! load_section {
    ($fmt:expr, $dest:expr, $source:expr) => {{
        let mut dest = $dest.write().unwrap();
        let loaded = bookmark_storage::load_from($source.by_ref())?;

        println!($fmt, loaded.len());

        dest.storage.extend(loaded.into_iter());
        dest.buffer.reset();
    }};
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

        load_section!("loaded {} infos", self.infos, lines);

        load_section!("loaded {} categories", self.categories, lines);

        load_section!("loaded {} bookmarks", self.bookmarks, lines);

        Ok(())
    }
}
