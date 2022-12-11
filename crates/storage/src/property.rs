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

impl std::error::Error for PropertyErr {}

#[derive(Debug, Clone)]
/// A property may be either a list of values or a single value
/// however the same property is always of the same type.
pub enum Property {
    /// List of values for this property.
    List(Vec<String>),
    /// Value of this property.
    Single(String),
}
