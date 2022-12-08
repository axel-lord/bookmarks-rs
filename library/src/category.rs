use crate::token;
use bookmark_storage::{ContentString, Field, ListField, Section, Storeable};
use std::{collections::HashMap, error::Error};

/// Type representing a category.
///
/// Categories are used to filter bookmarks based on some simple conditions, such
/// as whether the bookmark contains a substring.
///
/// The conditions can be both required, meaning all bookmarks in the category must fullfill them,
/// inclusive, meaning the bookmark may include it or another inclusive requirement, or meaning the
/// requirement is a perfect match with the bookmark.
#[derive(Debug, Storeable, Default)]
pub struct Category {
    #[line]
    line: ContentString,

    #[string]
    #[token(token::category::ID)]
    id: Field,

    #[string]
    #[title]
    #[token(token::category::NAME)]
    name: Field,

    #[string]
    #[token(token::category::DESCRIPTION)]
    description: Field,

    #[composite(identifier)]
    #[token(token::category::IDENTIFIER)]
    identifiers: ListField,

    #[composite(subcategory)]
    #[token(token::category::SUBCATEGORY)]
    subcategories: ListField,
}

/// Error type for issues creatin an [IdentifierContainer].
#[derive(Clone, Debug)]
pub struct IdentifierErr(String);

impl std::fmt::Display for IdentifierErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IdentifierErr {}

/// Type for representing the different requirement of a category.
#[derive(Clone, Debug, Default)]
pub struct IdentifierContainer<'a> {
    /// The required requirements.
    pub require: Vec<&'a str>,
    /// The full match requirements.
    pub whole: Vec<&'a str>,
    /// The optional requirements.
    pub include: Vec<&'a str>,
}

impl<'a> IdentifierContainer<'a> {
    /// Tally how many of each kind of requirement exist.
    pub fn tally(&self) -> HashMap<char, usize> {
        HashMap::from([
            ('(', self.include.len()),
            ('[', self.require.len()),
            ('<', self.whole.len()),
        ])
    }
}

impl Category {
    /// Get the requirements of a category.
    ///
    /// # Errors
    /// If one of the requirements is malformed.
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
    const ITEM_NAME: &'static str = "category";
    const TOKEN_END: &'static str = token::CATEGORY_END;
    const TOKEN_BEGIN: &'static str = token::CATEGORY_BEGIN;
}
