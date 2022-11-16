use crate::{token, Field};
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

pub const WHITESPACE_PADDED_GROUP: &str = r"\s*(.*?)\s*";

pub fn split_by_delim_to_ranges(list: &str) -> Vec<Range<usize>> {
    lazy_static! {
        static ref SPLIT_RE: Regex =
            Regex::new(&[r#"\s"#, token::DELIM, r#"\s|$"#].concat()).unwrap();
    }

    let mut next_start = 0;
    SPLIT_RE
        .find_iter(list)
        .map(move |m| {
            let r = next_start..m.start();
            next_start = m.end();
            r
        })
        .filter(|r| !r.is_empty())
        .collect()
}

pub fn split_list_field<'a>(list_field: &'a str) -> impl 'a + Iterator<Item = Field> {
    lazy_static! {
        static ref SPLIT_RE: Regex =
            Regex::new(&[r#"\s"#, token::DELIM, r#"\s|$"#].concat()).unwrap();
    }

    let mut next_start = 0;
    SPLIT_RE.find_iter(list_field).filter_map(move |m| {
        let range = next_start..m.start();
        next_start = m.end();

        if range.is_empty() {
            None
        } else {
            Some(range.into())
        }
    })
}
