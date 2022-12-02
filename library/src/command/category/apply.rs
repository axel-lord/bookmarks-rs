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

        let categories = categories.read().unwrap();

        let category = categories
            .storage
            .get(
                categories
                    .selected
                    .index()
                    .ok_or_else(|| CommandErr::Usage("no category selected".into()))?,
            )
            .unwrap();

        let criteria = category.identifier_container()?;
        let include_matcher = aho_corasick::AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .auto_configure(&criteria.include)
            .build(&criteria.include);

        bookmarks.write().unwrap().filter_in_place(|bookmark| {
            criteria.require.iter().all(|r| bookmark.url().contains(r))
                && (criteria.whole.iter().any(|v| *v == bookmark.url())
                    || include_matcher.is_match(bookmark.url()))
        });

        Ok(())
    })
}
