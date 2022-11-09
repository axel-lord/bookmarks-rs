use crate::token;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;

pub fn split_by_delim_to_ranges(list: &str) -> Vec<Range<usize>> {
    lazy_static! {
        static ref SPLIT_RE: Regex =
            Regex::new(&[r#"\s"#, token::DELIM, r#"\s|$"#].concat()).unwrap();
    }

    let mut next_start = 0;
    SPLIT_RE
        .find_iter(list)
        .map(|m| {
            let r = next_start..m.start();
            next_start = m.end();
            r
        })
        .filter(|r| !r.is_empty())
        .collect()
}
