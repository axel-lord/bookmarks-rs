use std::ops::Range;

#[derive(Debug, Clone)]
pub struct ContentString {
    is_appended_to: bool,
    string: String,
}

impl Default for ContentString {
    fn default() -> Self {
        ContentString {
            is_appended_to: false,
            string: Default::default(),
        }
    }
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

impl ContentString {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn take_any(self) -> String {
        self.string
    }

    pub fn ref_any(&self) -> &str {
        &self.string
    }

    pub fn is_appended_to(&self) -> bool {
        self.is_appended_to
    }

    pub fn append(&mut self, content: &str) -> Range<usize> {
        let begin = self.string.len();
        self.string += content;
        let end = self.string.len();

        self.is_appended_to = true;

        begin..end
    }
}
