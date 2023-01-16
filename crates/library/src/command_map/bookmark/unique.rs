use crate::{bookmark::Bookmark, shared};
use bookmark_command::{Command, CommandErr};

pub fn build(bookmarks: shared::BufferStorage<Bookmark>) -> Box<dyn Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "unique should be called without any arguments".into(),
            ));
        }

        let mut removed_count = 0usize;
        let mut bookmarks = bookmarks.write();

        bookmarks.storage.sort_by(|a, b| a.url().cmp(b.url()));
        bookmarks.storage.dedup_by(|a, b| {
            if a.url().eq_ignore_ascii_case(b.url()) {
                removed_count += 1;
                true
            } else {
                false
            }
        });

        println!("remvoved {removed_count} bookmarks");

        bookmarks.selected.clear();
        bookmarks.buffer.reset();

        Ok(())
    })
}
