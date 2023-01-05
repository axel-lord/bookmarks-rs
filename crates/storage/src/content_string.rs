use crate::Field;
use std::{fmt::Display, ops::Range, sync::Arc};

#[derive(Debug, Clone, Default)]
/// String that keeps track of whether or not it has been appended to
/// used by Storeable derives to store string data.
pub struct ContentString {
    is_appended_to: bool,
    content: Option<Content>,
}

#[derive(Debug, Default, Clone)]
enum Content {
    #[default]
    Empty,
    String(String),
    RcRange(Arc<str>, Range<usize>),
}

impl Content {
    fn from_string(value: String) -> Self {
        Self::String(value)
    }

    fn into_string(self) -> String {
        match self {
            Content::Empty => String::new(),
            Content::String(s) => s,
            Content::RcRange(rc, range) => String::from(&rc[range]),
        }
    }
}

impl ContentString {
    /// Create a new [`ContentString`], it is empty ad marked as not appended to.
    #[must_use]
    pub fn new() -> Self {
        ContentString::default()
    }

    /// Create a [`ContentString`] from a regular string, it is marked as not appended to.
    #[must_use]
    pub fn with_string(value: String) -> Self {
        ContentString {
            is_appended_to: false,
            content: Some(Content::from_string(value)),
        }
    }

    /// Create a [`ContentString`] from a an [Rc] and a Range, it is marked as not appended to.
    ///
    /// # Panics
    /// If the range is out of bound of the string.
    #[must_use]
    pub fn with_rc_range(rc: Arc<str>, range: Range<usize>) -> Self {
        assert!(rc.get(range.clone()).is_some());
        Self {
            is_appended_to: false,
            content: Some(Content::RcRange(rc, range)),
        }
    }

    /// Check whether or not the string has been appended to.
    #[must_use]
    pub fn has_been_pushed_to(&self) -> bool {
        self.is_appended_to
    }

    /// Push some more content onto the string and get the location
    /// of the pushed content.
    pub fn push(&mut self, content: &str) -> Range<usize> {
        let mut string = self
            .content
            .take()
            .expect("content should never be taken from outside of this take")
            .into_string();

        let begin = string.len();
        string += content;
        let end = string.len();

        self.content = Some(Content::from_string(string));

        self.is_appended_to = true;

        begin..end
    }

    /// Get content as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self
            .content
            .as_ref()
            .expect("content should never be taken from outside push")
        {
            Content::Empty => "",
            Content::String(s) => s,
            Content::RcRange(rc, range) => &rc[range.clone()],
        }
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

impl Display for ContentString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <str as Display>::fmt(self.as_str(), f)
    }
}

// Conversions

impl From<&str> for ContentString {
    fn from(content: &str) -> Self {
        ContentString::from(String::from(content))
    }
}

impl From<String> for ContentString {
    fn from(content: String) -> Self {
        Self::with_string(content)
    }
}

impl From<ContentString> for String {
    fn from(mut value: ContentString) -> Self {
        value
            .content
            .take()
            .expect("should not be None")
            .into_string()
    }
}

impl std::ops::Deref for ContentString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for ContentString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for ContentString {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
