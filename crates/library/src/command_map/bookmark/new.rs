use crate::{bookmark::Bookmark, shared};

use bookmark_command::{Command, CommandErr};

#[derive(Debug, Command)]
pub struct New {
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for New {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "new should be called without any arguments".into(),
            ));
        }

        let mut bookmarks = self.bookmarks.write();

        let index = bookmarks.storage.len();

        bookmarks.storage.push(Bookmark::new(
            "no url",
            "no info",
            std::iter::empty::<&str>(),
        ));

        bookmarks.buffer.reset();

        let mut selected = bookmarks.selected;
        selected.replace(index);

        println!(
            "added and selected:\n{index}. {:#}",
            bookmarks
                .storage
                .get(selected.index().expect("failed to get index of selected (should not happen as something was just selected)"))
                .ok_or_else(|| CommandErr::Execution(
                    "failed in using index of added bookmark".into()
                ))?
        );

        Ok(())
    }
}
