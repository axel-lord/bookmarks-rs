use bookmark_storage::{token, ContentString, ListField, Section, Storeable};

/// Used for storing info relevant for all bookmarks and categories, such as top level categories
/// and available tags.
#[derive(Debug, Storeable, Default)]
pub struct Info {
    #[line]
    line: ContentString,

    #[composite(category)]
    #[token(token::info::CATEGORY)]
    categories: ListField,

    #[composite(tag)]
    #[token(token::info::TAG)]
    tags: ListField,
}

impl Section for Info {
    const ITEM_NAME: &'static str = "info";
    const TOKEN_END: &'static str = token::INFO_END;
    const TOKEN_BEGIN: &'static str = token::INFO_BEGIN;
}
