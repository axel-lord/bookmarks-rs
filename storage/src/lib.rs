pub mod content_string;
pub mod pattern_match;
pub mod token;
use std::error::Error;

#[derive(Clone, Debug)]
pub enum ParseErr {
    Line(Option<String>, Option<usize>),
    Other(String),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Line(Some(l), None) => {
                write!(f, "could not parse line: {l}")
            }
            ParseErr::Line(Some(l), Some(i)) => {
                write!(f, "could not parse line {i}: {l}")
            }
            ParseErr::Line(None, None) => {
                write!(f, "could not parse anything")
            }
            ParseErr::Line(None, Some(i)) => {
                write!(f, "could not parse line {i}")
            }
            ParseErr::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Error for ParseErr {}

pub trait Storeable: Sized {
    fn is_edited(&self) -> bool;
    fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr>;
    fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, ParseErr>;
    fn to_line(&self) -> String;
}
