pub mod content_string;
pub mod load;
pub mod pattern_match;
pub mod save;
pub mod token;

pub use load::load;
pub use save::save;

use std::{
    error::Error,
    ops::{Add, AddAssign, Deref, DerefMut, Range},
};

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

impl From<std::io::Error> for ParseErr {
    fn from(err: std::io::Error) -> Self {
        ParseErr::Other(format!("{err}"))
    }
}

#[derive(Clone, Debug)]
pub enum PropertyErr {
    DoesNotExist(String),
}

impl std::fmt::Display for PropertyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyErr::DoesNotExist(ref prop) => {
                write!(f, "property {prop} does not exist in used capacity")
            }
        }
    }
}

impl Error for PropertyErr {}

#[derive(Debug, Clone)]
pub enum Property {
    List(Vec<String>),
    Single(String),
}

#[derive(Debug, Clone)]
pub struct Field(Range<usize>);

impl Field {
    pub fn new(from: usize, to: usize) -> Self {
        Self(from..to)
    }
}

impl Add<usize> for Field {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Field::new(self.0.start + rhs, self.0.end + rhs)
    }
}

impl AddAssign<usize> for Field {
    fn add_assign(&mut self, rhs: usize) {
        self.0.start += rhs;
        self.0.end += rhs;
    }
}

impl From<Range<usize>> for Field {
    fn from(r: Range<usize>) -> Self {
        Self(r)
    }
}

impl From<Field> for Range<usize> {
    fn from(f: Field) -> Self {
        f.0
    }
}

#[derive(Debug, Clone)]
pub struct ListField(Vec<Field>);

impl From<Vec<Range<usize>>> for ListField {
    fn from(r: Vec<Range<usize>>) -> Self {
        Self(r.into_iter().map(Field::from).collect())
    }
}

impl From<ListField> for Vec<Range<usize>> {
    fn from(f: ListField) -> Self {
        f.0.into_iter().map(Field::into).collect()
    }
}

impl Deref for ListField {
    type Target = Vec<Field>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ListField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ListField {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

pub trait Storeable: Sized {
    fn is_edited(&self) -> bool;
    fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr>;
    fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, ParseErr>;
    fn to_line(&self) -> String;

    fn get(&self, property: &str) -> Result<Property, PropertyErr>;
    fn set(&mut self, property: &str, value: Property) -> Result<(), PropertyErr>;
    fn push(&mut self, property: &str, value: &str) -> Result<(), PropertyErr>;
}

pub trait Section {
    fn token_begin() -> &'static str;
    fn token_end() -> &'static str;
    fn item_name() -> &'static str;
}

pub trait Listed: Storeable + Section {}

impl<T> Listed for T where T: Storeable + Section {}
