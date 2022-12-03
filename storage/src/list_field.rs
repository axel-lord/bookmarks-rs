use crate::Field;
use std::ops::{Deref, DerefMut, Range};

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
