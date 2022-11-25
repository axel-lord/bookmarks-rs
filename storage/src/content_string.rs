use crate::Field;
use std::ops::Range;

#[derive(Debug, Clone, Default)]
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

impl std::ops::Deref for ContentString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl ContentString {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn take(self) -> String {
        self.string
    }

    pub fn is_appended_to(&self) -> bool {
        self.is_appended_to
    }

    pub fn push(&mut self, content: &str) -> Range<usize> {
        let begin = self.string.len();
        self.string += content;
        let end = self.string.len();

        self.is_appended_to = true;

        begin..end
    }

    pub fn extend<'a>(
        &mut self,
        content: impl 'a + Iterator<Item = impl AsRef<str>>,
    ) -> Vec<Field> {
        content
            .map(|content| self.push(content.as_ref()).into())
            .collect()
    }
}
