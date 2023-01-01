use crate::{bookmark::Bookmark, category::Category, shared};
use bookmark_command::CommandErr;

pub fn build(
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
) -> Box<dyn bookmark_command::Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "apply should be called without any arguments".into(),
            ));
        }

        let categories = categories
            .read()
            .expect("failed to aquire read lock for categories");

        let category = categories
            .storage
            .get(
                categories
                    .selected
                    .index()
                    .ok_or_else(|| CommandErr::Usage("no category selected".into()))?,
            )
            .expect("failed to get selected category");

        category
            .apply(
                &mut bookmarks
                    .write()
                    .expect("failed to aquire write lock for bookmarks"),
            )
            .map_err(|err| CommandErr::Execution(format!("{err}")))?;
        Ok(())
    })
}
