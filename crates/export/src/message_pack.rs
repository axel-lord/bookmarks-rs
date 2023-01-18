use bookmark_command::{Command, CommandErr};
use bookmark_library::{shared, Bookmark, Category, Info};

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

        dbg!(&file_data);

        Ok(())
    })
}
