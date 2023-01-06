use crate::token;
use bookmark_storage::{ContentString, Field, ListField, Section, Storeable};

/// Type representing a bookmark.
/// Easily responsible for the most important data.
#[derive(Debug, Storeable, Default)]
pub struct Bookmark {
    #[line]
    line: ContentString,

    #[string]
    #[token(token::unsorted::URL)]
    url: Field,

    #[string]
    #[title]
    #[token(token::unsorted::INFO)]
    description: Field,

    #[composite(tag)]
    #[token(token::unsorted::TAG)]
    tags: ListField,
}

impl Bookmark {
    /// Get the amount of bytes used to store text used by bookmark.
    #[must_use]
    pub fn stored_length(&self) -> usize {
        self.line.as_bytes().len()
    }
}

impl AsRef<Bookmark> for Bookmark {
    fn as_ref(&self) -> &Bookmark {
        self
    }
}

impl Section for Bookmark {
    const ITEM_NAME: &'static str = "bookmark";
    const TOKEN_END: &'static str = token::unsorted::END;
    const TOKEN_BEGIN: &'static str = token::unsorted::BEGIN;
}
