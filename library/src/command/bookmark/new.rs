use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
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

        let mut bookmarks = self.bookmarks.write().unwrap();

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
                .get(selected.index().unwrap())
                .ok_or_else(|| CommandErr::Execution(
                    "failed in using index of added bookmark".into()
                ))?
        );

        Ok(())
    }
}
