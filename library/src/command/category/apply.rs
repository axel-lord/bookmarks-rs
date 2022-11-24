use std::borrow::BorrowMut;

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    shared,
};

pub fn build(
    bookmarks: shared::BufferStorage<Bookmark>,
    categories: shared::BufferStorage<Category>,
) -> Box<dyn Command> {
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "apply should be called without any arguments".into(),
            ));
        }

        let category_storage = categories.storage.borrow();
        let category = categories
            .selected
            .get(&category_storage)
            .ok_or_else(|| CommandErr::Usage("no category selected".into()))?;

        let criteria = category.identifier_container()?;

        bookmarks
            .buffer
            .filter_in_place(&bookmarks.storage, |_bookmark| true);

        Ok(())
    })
}
