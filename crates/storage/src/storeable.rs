use crate::{ContentString, ParseErr, Property, PropertyErr};

/// Trait used to mark a type as serializable.
pub trait Storeable: Sized {
    /// Whether or not the type has been edited.
    fn is_edited(&self) -> bool;

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

    /// Construct an instance from a ready [ContentString].
    ///
    /// # Errors
    /// If the [ContentString] cannot be parsed to the [Storeable].
    fn with_content_string(line: ContentString, line_num: Option<usize>) -> Result<Self, ParseErr>;

    /// Construct an instance from a string.
    ///
    /// # Errors
    /// If the string cannot be parsed to the [Storeable].
    fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr> {
        Self::with_content_string(line.into(), line_num)
    }

    /// Construct an instance from a string slice.
    ///
    /// # Errors
    /// If the string slice cannot be parsed to the [Storeable].
    fn with_str(line: impl Into<String>, line_num: Option<usize>) -> Result<Self, ParseErr> {
        Self::with_string(line.into(), line_num)
    }
}
