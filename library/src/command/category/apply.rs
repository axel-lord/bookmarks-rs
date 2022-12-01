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

        let apply_category = |category: &Category| -> Result<(), CommandErr> {
            let criteria = category.identifier_container()?;
            let include_matcher = aho_corasick::AhoCorasickBuilder::new()
                .ascii_case_insensitive(true)
                .auto_configure(&criteria.include)
                .build(&criteria.include);

            bookmarks
                .buffer
                .filter_in_place(&bookmarks.storage, |bookmark| {
                    criteria.require.iter().all(|r| bookmark.url().contains(r))
                        && (criteria.whole.iter().any(|v| *v == bookmark.url())
                            || include_matcher.is_match(bookmark.url()))
                });
            Ok(())
        };

        apply_category(
            categories
                .storage
                .read()
                .get(
                    categories
                        .selected
                        .index()
                        .ok_or_else(|| CommandErr::Usage("no category selected".into()))?,
                )
                .unwrap(),
        )?;

        Ok(())
    })
}
