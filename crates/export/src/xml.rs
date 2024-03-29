use bookmark_command::{Command, CommandErr};
use bookmark_library::{shared, Bookmark, Category, Info};
use std::{
    fs::File,
    io::{prelude::*, BufWriter},
};
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
                "export xml should be given a file path".into(),
            ));
        }

        let infos = infos.read();
        let categories = categories.read();
        let bookmarks = bookmarks.read();

        File::create(args[0].clone())?
            .pipe(BufWriter::new)
            .write_all(
                FileData::from_slices(&infos.storage, &categories.storage, &bookmarks.storage)
                    .pipe_ref(quick_xml::se::to_string)
                    .map_err(|err| CommandErr::Execution(err.to_string()))?
                    .as_bytes(),
            )
            .map_err(|err| CommandErr::Execution(err.to_string()))?;

        Ok(())
    })
}
