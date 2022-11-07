use super::token;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Bookmark<'a> {
    pub url: &'a str,
    pub description: &'a str,
    pub tags: Vec<&'a str>,
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

impl<'a> Bookmark<'a> {
    pub fn with_str(line: &'a str, line_num: Option<usize>) -> Result<Self, BookmarkErr> {
        let err = || BookmarkErr::LineParseFailure(line.into(), line_num);

        let url_start = line.find(token::unsorted::URL).ok_or_else(err)?;
        let description_start = line.find(token::unsorted::DESCRIPTION).ok_or_else(err)?;
        let tag_start = line.find(token::unsorted::TAG).ok_or_else(err)?;

        Ok(Bookmark {
            url: &line[url_start + token::unsorted::URL.len()..description_start].trim(),
            description: &line[description_start + token::unsorted::DESCRIPTION.len()..tag_start]
                .trim(),
            tags: line[tag_start + token::unsorted::TAG.len()..]
                .split("<,>")
                .map(str::trim)
                .collect(),
        })
    }
}

impl<'a> std::fmt::Display for Bookmark<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.url, self.description)?;

        let mut tag_iter = self.tags.iter();
        if let Some(t) = tag_iter.next() {
            write!(f, " | {t}")?;
        };

        for t in tag_iter {
            write!(f, ", {t}")?
        }

        Ok(())
    }
}
