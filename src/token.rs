pub const UNSORTED_BEGIN: &str = "#UNSORTED_BEGIN";
pub const UNSORTED_END: &str = "#UNSORTED_END";
pub const DELIM: &str = "<,>";

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
