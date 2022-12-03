use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    info::Info,
    shared,
};
use bookmark_command::Command;
use bookmark_storage::Listed;
use std::{fs::File, io::BufWriter};

#[derive(Debug, Command)]
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

        let buffer_storage = self.buffer_storage.write().unwrap();

        bookmark_storage::save(
            &mut BufWriter::new(File::create(&args[0])?),
            buffer_storage
                .buffer
                .iter()
                .map(|i| buffer_storage.storage.get(i))
                .take_while(Option::is_some)
                .map(Option::unwrap),
        )?;

        Ok(())
    }
}

#[derive(Debug, Command)]
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

        bookmark_storage::save(&mut writer, self.infos.read().unwrap().storage.iter())?;

        bookmark_storage::save(&mut writer, self.categories.read().unwrap().storage.iter())?;

        bookmark_storage::save(&mut writer, self.bookmarks.read().unwrap().storage.iter())?;

        Ok(())
    }
}
