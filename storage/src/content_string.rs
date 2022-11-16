use std::ops::Range;

#[derive(Debug, Clone)]
pub enum ContentString {
    AppendedTo(String),
    UnappendedTo(String),
}

impl ContentString {
    pub fn take_any(self) -> String {
        dbg!("take");
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    pub fn ref_any(&self) -> &str {
        dbg!("ref");
        dbg!(self);
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    pub fn is_appended_to(&self) -> bool {
        match self {
            Self::UnappendedTo(_) => false,
            Self::AppendedTo(_) => true,
        }
    }

    pub fn append(self, content: &str) -> (Self, Range<usize>) {
        dbg!("append init");
        dbg!(content);
        let mut existing = self.take_any();

        let begin = existing.len();
        existing += content;
        let end = existing.len();

        dbg!((ContentString::AppendedTo(existing), begin..end))
    }
}
