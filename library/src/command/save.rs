use std::{fs::File, io::BufWriter};

use bookmark_storage::Listed;

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    info::Info,
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save<T> {
    buffer_storage: shared::BufferStorage<T>,
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

        let storage = self.buffer_storage.storage.read().unwrap();
        bookmark_storage::save(
            &mut BufWriter::new(File::create(&args[0])?),
            self.buffer_storage
                .buffer
                .read()
                .unwrap()
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
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for SaveAll {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        let mut writer = BufWriter::new(File::create(&args[0])?);

        bookmark_storage::save(&mut writer, self.infos.storage.read().unwrap().iter())?;

        bookmark_storage::save(&mut writer, self.categories.storage.read().unwrap().iter())?;

        bookmark_storage::save(&mut writer, self.bookmarks.storage.read().unwrap().iter())?;

        Ok(())
    }
}
