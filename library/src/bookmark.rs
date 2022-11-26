use bookmark_storage::{token, ContentString, Field, ListField, Section};

#[derive(Debug, bookmark_derive::Storeable, Default)]
pub struct Bookmark {
    #[line]
    line: ContentString,

    #[string]
    #[token(token::unsorted::URL)]
    url: Field,

    #[string]
    #[title]
    #[token(token::unsorted::DESCRIPTION)]
    description: Field,

    #[composite(tag)]
    #[token(token::unsorted::TAG)]
    tags: ListField,
}

impl Section for Bookmark {
    const ITEM_NAME: &'static str = "bookmark";
    const TOKEN_END: &'static str = token::UNSORTED_END;
    const TOKEN_BEGIN: &'static str = token::UNSORTED_BEGIN;
}
