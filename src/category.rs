use std::{error::Error, ops::Range};

use crate::{token, ContentString};

#[derive(Debug)]
pub struct Category {
    line: Option<ContentString>,
    id: Range<usize>,
    name: Range<usize>,
    description: Range<usize>,
}

#[derive(Clone, Debug)]
pub enum CategoryErr {
    LineParseFailure(String, Option<usize>),
}

impl std::fmt::Display for CategoryErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryErr::LineParseFailure(l, None) => {
                write!(f, "line parse failure on line \"{l}\"")
            }
            CategoryErr::LineParseFailure(l, Some(i)) => {
                write!(f, "line parse failure on line {i} \"{l}\"")
            }
        }
    }
}

impl Error for CategoryErr {}

impl Clone for Category {
    fn clone(&self) -> Self {
        let line = self.to_line();
        Self::with_str(line, None).unwrap()
    }
}

impl Category {
    pub fn with_str(line: String, line_num: Option<usize>) -> Result<Self, CategoryErr> {
        Err(CategoryErr::LineParseFailure(line, line_num))
    }

    pub fn to_line(&self) -> String {
        format!(
            "{} {} {} {} {} {} {}",
            token::category::ID,
            self.id(),
            token::category::NAME,
            self.name(),
            token::category::DESCRIPTION,
            self.description(),
            token::category::IDENTIFIER
        )
    }

    pub fn is_edited(&self) -> bool {
        self.line.as_ref().unwrap().is_appended_to()
    }

    pub fn id(&self) -> &str {
        &self.raw_line()[self.id.clone()]
    }

    pub fn name(&self) -> &str {
        &self.raw_line()[self.name.clone()]
    }

    pub fn description(&self) -> &str {
        &self.raw_line()[self.description.clone()]
    }

    fn raw_line(&self) -> &str {
        self.line.as_ref().unwrap().ref_any()
    }
}
