use crate::{
    load::ParseErr,
    pattern_match::{self, split_by_delim_to_ranges},
    token, ContentString,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

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

#[derive(Debug)]
pub struct Bookmark {
    line: Option<ContentString>,
    url: Range<usize>,
    description: Range<usize>,
    tag: Range<usize>,
    tags: Vec<Range<usize>>,
}

impl Clone for Bookmark {
    fn clone(&self) -> Self {
        Self::with_string(self.to_line(), None).unwrap()
    }
}

impl From<Bookmark> for String {
    fn from(b: Bookmark) -> Self {
        b.to_line()
    }
}

impl Bookmark {
    pub fn new<'a>(url: &str, description: &str, tags: impl Iterator<Item = &'a str>) -> Self {
        Self::with_string(Self::create_line(url, description, tags), None).unwrap()
    }

    pub fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, ParseErr> {
        Self::with_string(line.into(), line_num)
    }

    pub fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(
                &[
                    r#"^"#,
                    token::unsorted::URL,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::unsorted::DESCRIPTION,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::unsorted::TAG,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    r"$"
                ]
                .concat()
            )
            .unwrap();
        }

        let err = || ParseErr::Line(Some(line.clone()), line_num);

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

        let tags = split_by_delim_to_ranges(&line[tag.clone()]);

        Ok(Bookmark {
            line: Some(ContentString::UnappendedTo(line)),
            url,
            description,
            tag,
            tags,
        })
    }

    pub fn add_tag(&mut self, tag: &str) {
        let (content_string, range) = self.line.take().unwrap().append(tag);

        self.line = Some(content_string);
        self.tags.push(range);
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
        if let Some(ContentString::UnappendedTo(line)) = self.line.as_ref() {
            line.clone()
        } else {
            Self::create_line(self.url(), self.description(), self.tags())
        }
    }

    pub fn is_edited(&self) -> bool {
        self.line.as_ref().unwrap().is_appended_to()
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
