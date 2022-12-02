use crate::{
    bookmark::Bookmark,
    command::{Command, CommandErr},
    reset::ResetValues,
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct New {
    bookmarks: shared::BufferStorage<Bookmark>,
    reset_values: ResetValues,
}

impl Command for New {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "new should be called without any arguments".into(),
            ));
        }

        let mut bookmarks = self.bookmarks.storage.write().unwrap();

        let index = bookmarks.len();

        bookmarks.push(Bookmark::new(
            "no url",
            "no info",
            std::iter::empty::<&str>(),
        ));

        self.reset_values.reset();

        let mut selected = self.bookmarks.selected.write().unwrap();
        selected.replace(index);

        println!(
            "added and selected:\n{index}. {:#}",
            bookmarks
                .get(selected.index().unwrap())
                .ok_or_else(|| CommandErr::Execution(
                    "failed in using index of added bookmark".into()
                ))?
        );

        Ok(())
    }
}
