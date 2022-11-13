use crate::{token, ContentString};
use bookmark_storage::Storeable;
use std::ops::Range;

#[derive(Debug, bookmark_derive::Storeable)]
pub struct Bookmark {
    #[line]
    line: Option<ContentString>,

    #[string]
    #[token(token::unsorted::URL)]
    url: Range<usize>,

    #[string]
    #[token(token::unsorted::DESCRIPTION)]
    description: Range<usize>,

    #[composite(tag)]
    #[token(token::unsorted::TAG)]
    tags: Vec<Range<usize>>,
    tag: Range<usize>,
}

impl std::fmt::Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.url(), self.description())?;

        let mut tag_iter = self.tags();
        if let Some(t) = tag_iter.next() {
            write!(f, " | {}", t)?;
        };

        for t in tag_iter {
            write!(f, ", {}", t)?
        }

        Ok(())
    }
}
