use std::ops::{Add, AddAssign, Range};

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
