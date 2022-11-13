use crate::{token, ContentString};
use bookmark_storage::{Section, Storeable};
use std::{collections::HashMap, error::Error, ops::Range};

#[derive(Debug, bookmark_derive::Storeable)]
pub struct Category {
    #[line]
    line: Option<ContentString>,

    #[string]
    #[token(token::category::ID)]
    id: Range<usize>,

    #[string]
    #[token(token::category::NAME)]
    name: Range<usize>,

    #[string]
    #[token(token::category::DESCRIPTION)]
    description: Range<usize>,

    #[composite(identifier)]
    #[token(token::category::IDENTIFIER)]
    identifiers: Vec<Range<usize>>,
    identifier: Range<usize>,

    #[composite(subcategory)]
    #[token(token::category::SUBCATEGORY)]
    subcategories: Vec<Range<usize>>,
    subcategory: Range<usize>,
}

#[derive(Clone, Debug)]
pub struct IdentifierErr(String);

impl std::fmt::Display for IdentifierErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IdentifierErr {}

#[derive(Clone, Debug, Default)]
pub struct IdentifierContainer<'a> {
    pub require: Vec<&'a str>,
    pub whole: Vec<&'a str>,
    pub include: Vec<&'a str>,
}

impl<'a> IdentifierContainer<'a> {
    pub fn tally(&self) -> HashMap<char, usize> {
        HashMap::from([
            ('(', self.include.len()),
            ('[', self.require.len()),
            ('<', self.whole.len()),
        ])
    }
}

impl Category {
    pub fn identifier_container<'a>(&'a self) -> Result<IdentifierContainer<'a>, IdentifierErr> {
        let mut identifier_container: IdentifierContainer<'a> = Default::default();

        for identifier in self.identifiers() {
            let ty = identifier.get(..1).ok_or_else(|| {
                IdentifierErr(format!(
                    "identifier \"{}\" does not start with an ascii character",
                    identifier
                ))
            })?;

            // ok since above succeeded
            let identifier_content = &identifier[1..];

            match ty {
                "(" => {
                    identifier_container.include.push(identifier_content);
                }
                "<" => {
                    identifier_container.whole.push(identifier_content);
                }
                "[" => {
                    identifier_container.require.push(identifier_content);
                }
                spec => {
                    return Err(IdentifierErr(format!(
                        "invalid identifier specifier '{}' in identifier: {}",
                        spec, identifier
                    )));
                }
            }
        }

        Ok(identifier_container)
    }
}

impl Section for Category {
    fn token_end() -> &'static str {
        token::CATEGORY_END
    }

    fn token_begin() -> &'static str {
        token::CATEGORY_BEGIN
    }

    fn item_name() -> &'static str {
        "category"
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.name(),
            self.description(),
            self.id()
        )?;

        if !self.identifiers.is_empty() {
            write!(
                f,
                "\nidentifiers: {}",
                &self.identifiers().collect::<Vec<&str>>().join(", ")
            )?
        }

        if !self.subcategories.is_empty() {
            write!(
                f,
                "\nsubcategories: {}",
                &self.subcategories().collect::<Vec<&str>>().join(", ")
            )?
        }

        Ok(())
    }
}
