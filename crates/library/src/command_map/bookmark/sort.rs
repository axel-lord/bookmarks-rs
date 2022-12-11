use crate::{bookmark::Bookmark, shared};
use bookmark_command::{Command, CommandErr};

pub fn build(bookmarks: shared::BufferStorage<Bookmark>) -> Box<dyn Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "sort should be called without any arguments".into(),
            ));
        }

        let mut bookmarks = bookmarks.write().unwrap();

        bookmarks
            .storage
            .sort_by(|a, b| a.url().partial_cmp(b.url()).unwrap());

        bookmarks.buffer.reset();
        bookmarks.selected.clear();

        Ok(())
    })
}
