use crate::Field;
use std::{fmt::Display, ops::Range};

#[derive(Debug, Clone, Default)]
/// String that keeps track of whether or not it has been appended to
/// used by Storeable derives to store string data.
pub struct ContentString {
    is_appended_to: bool,
    string: String,
}

impl From<&str> for ContentString {
    fn from(content: &str) -> Self {
        ContentString::from(String::from(content))
    }
}

impl From<String> for ContentString {
    fn from(content: String) -> Self {
        ContentString {
            is_appended_to: false,
            string: content,
        }
    }
}

impl From<ContentString> for String {
    fn from(value: ContentString) -> Self {
        String::from(<ContentString as AsRef<str>>::as_ref(&value))
    }
}

impl Display for ContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <str as Display>::fmt(self, f)
    }
}

impl std::ops::Deref for ContentString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl AsRef<str> for ContentString {
    fn as_ref(&self) -> &str {
        self
    }
}

impl AsRef<[u8]> for ContentString {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl ContentString {
    /// Create a new ContentString it is empty and
    /// marked as not appended to.
    pub fn new() -> Self {
        Default::default()
    }

    /// Consume the ContentString and get a regular string from it.
    pub fn take(self) -> String {
        self.string
    }

    /// Check whether or not the string has been appended to.
    pub fn is_appended_to(&self) -> bool {
        self.is_appended_to
    }

    /// Push some more content onto the string and get the location
    /// of the pushed content.
    pub fn push(&mut self, content: &str) -> Range<usize> {
        let begin = self.string.len();
        self.string += content;
        let end = self.string.len();

        self.is_appended_to = true;

        begin..end
    }

    /// Push multiple pieces of content and get their locations.
    pub fn extend<'a>(
        &mut self,
        content: impl 'a + Iterator<Item = impl AsRef<str>>,
    ) -> Vec<Field> {
        content
            .map(|content| self.push(content.as_ref()).into())
            .collect()
    }
}
