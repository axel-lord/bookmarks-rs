use crate::{token, ContentString};
use bookmark_storage::Storeable;
use lazy_static::lazy_static;
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

impl Bookmark {
    pub fn new<'a>(url: &str, description: &str, tags: impl Iterator<Item = &'a str>) -> Self {
        Self::with_string(Self::create_line(url, description, tags), None).unwrap()
    }

    pub fn add_tag(&mut self, tag: &str) {
        let (content_string, range) = self.line.take().unwrap().append(tag);

        self.line = Some(content_string);
        self.tags.push(range);
    }

    fn create_line<'a>(
        url: &str,
        description: &str,
        tags: impl Iterator<Item = &'a str>,
    ) -> String {
        format!(
            "{} {} {} {} {} {}",
            token::unsorted::URL,
            url,
            token::unsorted::DESCRIPTION,
            description,
            token::unsorted::TAG,
            tags.collect::<Vec<&str>>()
                .join(&[" ", token::DELIM, " "].concat()),
        )
    }
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
