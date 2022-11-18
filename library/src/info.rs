use bookmark_storage::{content_string::ContentString, token, ListField, Section};

#[derive(Debug, bookmark_derive::Storeable)]
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
