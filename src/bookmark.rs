use super::token;
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, ops::Range};

#[derive(Debug, Clone)]
enum ContentString {
    AppendedTo(String),
    UnappendedTo(String),
}

#[macro_export]
macro_rules! append_chain {
    ($dst:expr, $($x:expr),*) => {
        {
            $(
                $dst += $x;
            )+
        }
    };
}

impl ContentString {
    fn take_any(self) -> String {
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    fn ref_any(&self) -> &str {
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    fn is_appended_to(&self) -> bool {
        match self {
            Self::UnappendedTo(_) => false,
            Self::AppendedTo(_) => true,
        }
    }
}

#[derive(Debug)]
pub struct Bookmark {
    url: Range<usize>,
    description: Range<usize>,
    tag: Range<usize>,
    tags: Vec<Range<usize>>,
    line: Option<ContentString>,
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

impl Clone for Bookmark {
    fn clone(&self) -> Self {
        let line = self.to_line();
        Self::with_str(line, None).unwrap()
    }
}

impl Bookmark {
    pub fn with_str(line: String, line_num: Option<usize>) -> Result<Self, BookmarkErr> {
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

        let err = || BookmarkErr::LineParseFailure(line.clone(), line_num);

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
            line: Some(ContentString::UnappendedTo(line)),
            url,
            description,
            tag,
            tags,
        })
    }

    pub fn add_tag(&mut self, tag: &str) {
        let mut string = self.line.take().unwrap().take_any();

        let start = string.len();
        string += tag;
        let end = string.len();

        self.line = Some(ContentString::AppendedTo(string));
        self.tags.push(start..end);
    }

    pub fn url(&self) -> &str {
        &self.raw_line()[self.url.clone()]
    }

    pub fn description(&self) -> &str {
        &self.raw_line()[self.description.clone()]
    }

    pub fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags
            .iter()
            .map(|r| &self.raw_line()[self.tag.clone()][r.clone()])
    }

    pub fn to_line(&self) -> String {
        let mut s = format!(
            "{} {} {} {} {}",
            token::unsorted::URL,
            self.url(),
            token::unsorted::DESCRIPTION,
            self.description(),
            token::unsorted::TAG
        );

        let mut tag_iter = self.tags();
        if let Some(t) = tag_iter.next() {
            append_chain!(s, " ", t);
        }

        for t in tag_iter {
            append_chain!(s, " ", token::unsorted::TAG_DELIM, " ", t);
        }

        s
    }

    pub fn is_edited(&self) -> bool {
        self.line.as_ref().unwrap().is_appended_to()
    }

    fn raw_line(&self) -> &str {
        self.line.as_ref().unwrap().ref_any()
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
