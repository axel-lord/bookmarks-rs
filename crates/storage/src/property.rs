use thiserror::Error;

#[derive(Clone, Debug, Error)]
/// Error type for issues appearing when accessing properties.
pub enum PropertyErr {
    /// If the property does not exist and a message.
    #[error("property {0} does not exist as expected type of property")]
    DoesNotExist(String),
}

#[derive(Debug, Clone)]
/// A property may be either a list of values or a single value
/// however the same property is always of the same type.
pub enum Property {
    /// List of values for this property.
    List(Vec<String>),
    /// Value of this property.
    Single(String),
}
