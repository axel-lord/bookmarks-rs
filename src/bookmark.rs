use super::token;
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, ops::Range};

#[derive(Debug, Clone)]
pub struct Bookmark {
    url: Range<usize>,
    description: Range<usize>,
    tag: Range<usize>,
    tags: Vec<Range<usize>>,
    line: String,
}

#[derive(Clone, Debug)]
pub enum BookmarkErr {
    LineParseFailure(String, Option<usize>),
}

impl std::fmt::Display for BookmarkErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BookmarkErr::LineParseFailure(l, None) => {
                write!(f, "line parse failure on line \"{l}\"")
            }
            BookmarkErr::LineParseFailure(l, Some(i)) => {
                write!(f, "line parse failure on line {i} \"{l}\"")
            }
        }
    }
}

impl Error for BookmarkErr {}

impl Bookmark {
    pub fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, BookmarkErr> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(
                &[
                    r#"^"#,
                    token::unsorted::URL,
                    r#"\s*(.*?)\s*"#,
                    token::unsorted::DESCRIPTION,
                    r#"\s*(.*?)\s*"#,
                    token::unsorted::TAG,
                    r#"\s*(.*?)\s*$"#
                ]
                .concat()
            )
            .unwrap();
            static ref TAG_RE: Regex =
                Regex::new(&[r#"\s"#, token::unsorted::TAG_DELIM, r#"\s|$"#].concat()).unwrap();
        }

        let err = || BookmarkErr::LineParseFailure(line.into(), line_num);

        let line: String = line.into();

        let captures = LINE_RE.captures(&line).ok_or_else(err)?;

        let url = captures
            .get(1)
            .and_then(|c| Some(c.range()))
            .ok_or_else(err)?;

        let description = captures
            .get(2)
            .and_then(|c| Some(c.range()))
            .ok_or_else(err)?;

        let tag = captures
            .get(3)
            .and_then(|c| Some(c.range()))
            .ok_or_else(err)?;

        let mut last_start = 0;
        let tags = TAG_RE
            .find_iter(&line[tag.clone()])
            .map(|m| {
                let r = last_start..m.start();
                last_start = m.end();
                r
            })
            .filter(|r| !r.is_empty())
            .collect();

        Ok(Bookmark {
            line,
            url,
            description,
            tag,
            tags,
        })
    }
}

impl std::fmt::Display for Bookmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {}",
            &self.line[self.url.clone()],
            &self.line[self.description.clone()],
        )?;

        let mut tag_iter = self.tags.iter();
        if let Some(t) = tag_iter.next() {
            write!(f, " | {}", &self.line[self.tag.clone()][t.clone()])?;
        };

        for t in tag_iter {
            write!(f, ", {}", &self.line[self.tag.clone()][t.clone()])?
        }

        Ok(())
    }
}
