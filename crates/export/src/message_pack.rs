use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bookmark_command::{Command, CommandErr};
use bookmark_library::{shared, Bookmark, Category, Info};
use tap::Pipe;

use crate::FileData;

pub fn build(
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
) -> Box<dyn Command> {
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "export mp should be given a file path".into(),
            ));
        }

        let infos = infos.read();
        let categories = categories.read();
        let bookmarks = bookmarks.read();

        let file_data =
            FileData::from_slices(&infos.storage, &categories.storage, &bookmarks.storage);

        File::create(&args[0])?.pipe(BufWriter::new).write_all(
            &rmp_serde::to_vec(&file_data)
                .map_err(|err| err.to_string().pipe(CommandErr::Execution))?,
        )?;

        Ok(())
    })
}
