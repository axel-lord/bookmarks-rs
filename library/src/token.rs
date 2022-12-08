//! Tokens used to parse bookmarks, info and categories.
#![allow(missing_docs)]

pub mod info {
    pub const CATEGORY: &str = "<category>";
    pub const TAG: &str = "<tag>";
}

pub mod unsorted {
    pub const URL: &str = "<url>";
    pub const DESCRIPTION: &str = "<info>";
    pub const TAG: &str = "<tag>";
}

pub mod category {
    pub const ID: &str = "<id>";
    pub const DESCRIPTION: &str = "<desc>";
    pub const NAME: &str = "<name>";
    pub const IDENTIFIER: &str = "<identifier>";
    pub const SUBCATEGORY: &str = "<sub>";
}

pub const UNSORTED_BEGIN: &str = "#UNSORTED_BEGIN";
pub const UNSORTED_END: &str = "#UNSORTED_END";
pub const CATEGORY_BEGIN: &str = "#CATEGORY_BEGIN";
pub const CATEGORY_END: &str = "#CATEGORY_END";
pub const INFO_BEGIN: &str = "#INFO_BEGIN";
pub const INFO_END: &str = "#INFO_END";
