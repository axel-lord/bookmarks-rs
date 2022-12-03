//! Crate used for definitions of items needed to serialize and deserialize bookmarks,
//! categories and info.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::missing_crate_level_docs
)]

/// [ContentString] related functionality.
pub mod content_string;

/// Functionality for loading [Listed] types.
pub mod load;

/// Helpers for pattern matching.
pub mod pattern_match;

/// Functionality for saving [Listed] types.
pub mod save;

/// Constants used for saving and loading.
pub mod token;

pub use content_string::ContentString;
pub use load::load;
pub use load::load_from;
pub use save::save;

use std::{
    error::Error,
    iter::FromIterator,
    ops::{Add, AddAssign, Deref, DerefMut, Range},
};

#[derive(Clone, Debug)]
/// Erros representing failure to parse some content.
pub enum ParseErr {
    /// If some line was unsuccessfully parsed optionally has which line and/or a
    /// message.
    Line(Option<String>, Option<usize>),
    /// Some other issue parsing with a message.
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
/// Error type for issues appearing when accessing properties.
pub enum PropertyErr {
    /// If the property does not exist and a message.
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
/// A property may be either a list of values or a single value
/// however the same property is always of the same type.
pub enum Property {
    /// List of values for this property.
    List(Vec<String>),
    /// Value of this property.
    Single(String),
}

#[derive(Debug, Clone)]
/// A field in a serializeable struct.
pub struct Field(Range<usize>);

impl Field {
    /// Create a new field from two positions in a string slice.
    pub fn new(from: usize, to: usize) -> Self {
        Self(from..to)
    }

    /// Get the field as a string slice existing in another string slice.
    pub fn get<'a>(&self, from: &'a str) -> &'a str {
        &from[self.0.clone()]
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

impl Default for Field {
    fn default() -> Self {
        Self(0..0)
    }
}

#[derive(Debug, Clone, Default)]
/// A field that is a list of values in a serializeable struct.
pub struct ListField(Vec<Field>);

impl From<Vec<Range<usize>>> for ListField {
    fn from(r: Vec<Range<usize>>) -> Self {
        Self(r.into_iter().map(Field::from).collect())
    }
}
impl From<Vec<Field>> for ListField {
    fn from(r: Vec<Field>) -> Self {
        Self(r)
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

impl FromIterator<Field> for ListField {
    fn from_iter<T: IntoIterator<Item = Field>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl FromIterator<Range<usize>> for ListField {
    fn from_iter<T: IntoIterator<Item = Range<usize>>>(iter: T) -> Self {
        Self(iter.into_iter().map(Field::from).collect())
    }
}

impl ListField {
    /// Create a new empty [ListField].
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Get an iterator of the contained values of a [ListField] as
    /// string slices existing in another string slice.
    pub fn get<'a>(&'a self, from: &'a str) -> impl Iterator<Item = &'a str> {
        self.0.iter().map(|f| f.get(from))
    }
}

/// Join and iterator of string slices into a single string delimited by [token::DELIM].
pub fn join_with_delim(mut fields: impl Iterator<Item = impl AsRef<str>>) -> String {
    use lazy_static::lazy_static;
    lazy_static! {
        static ref DELIM: String = format!(" {} ", token::DELIM);
    }

    let mut out = String::new();

    for i in fields.by_ref().take(1) {
        out += i.as_ref();
    }

    for i in fields {
        out += &DELIM;
        out += i.as_ref();
    }

    out
}

pub use bookmark_derive::Storeable;

/// Trait used to mark a type as serializable.
pub trait Storeable: Sized {
    /// Whether or not the type has been edited.
    fn is_edited(&self) -> bool;

    /// Construct an instance from a string.
    ///
    /// # Errors
    /// If the string cannot be parsed to the type.
    fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr>;

    /// Get a string from an instance.
    fn to_line(&self) -> String;

    /// Get a property from the instance.
    ///
    /// # Errors
    /// If the property does not exist.
    fn get(&self, property: &str) -> Result<Property, PropertyErr>;

    /// Set a property on the instance.
    ///
    /// # Errors
    /// If the property does not exist, or if the wrong kind
    /// of poperty is passed.
    fn set(&mut self, property: &str, value: Property) -> Result<&mut Self, PropertyErr>;

    /// Push a value onto a list property on the instance.
    ///
    /// # Errors
    /// If the property does not exist, or if it is not a [Property::List]
    fn push(&mut self, property: &str, value: &str) -> Result<&mut Self, PropertyErr>;

    /// Construct an instance from a string slice.
    ///
    /// # Errors
    /// If the string slice cannot be parsed to the type.
    fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, ParseErr> {
        Self::with_string(line.into(), line_num)
    }
}

/// Trait used to mark a type as serializable in sections.
pub trait Section {
    /// Name of the type.
    const ITEM_NAME: &'static str;
    /// Content of the line signaling the beginning of the section.
    const TOKEN_BEGIN: &'static str;
    /// Content of the line signaling the end of the section.
    const TOKEN_END: &'static str;
}

/// Trait for types that imlement both [Section] ans [Storeable].
pub trait Listed: Storeable + Section {}

impl<T> Listed for T where T: Storeable + Section {}

/// Write contents of a string slice iterator delimited by [token::DELIM].
///
/// # Errors
/// If a write operation failed.
pub fn write_delim_list(
    f: &mut std::fmt::Formatter<'_>,
    mut iter: impl Iterator<Item = impl AsRef<str>>,
) -> std::fmt::Result {
    for i in iter.by_ref().take(1) {
        write!(f, " {} ", i.as_ref())?;
    }
    for i in iter {
        write!(f, "{} {} ", token::DELIM, i.as_ref())?;
    }
    Ok(())
}

/// Write contents of a list field.
///
/// # Errors
/// If a write operation failed.
pub fn write_list_field(
    f: &mut std::fmt::Formatter<'_>,
    mut iter: impl Iterator<Item = impl AsRef<str>>,
) -> std::fmt::Result {
    write!(f, "[")?;
    for i in iter.by_ref().take(1) {
        write!(f, "{}", i.as_ref())?;
    }
    for i in iter {
        write!(f, ", {}", i.as_ref())?;
    }
    write!(f, "]")?;
    Ok(())
}
