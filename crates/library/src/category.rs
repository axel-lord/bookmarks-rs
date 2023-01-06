use crate::{container::BufferStorage, token, Bookmark};
use bookmark_storage::{ContentString, Field, ListField, Section, Storeable};
use std::collections::HashMap;
use thiserror::Error;

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
    #[token(token::category::DESC)]
    description: Field,

    #[composite(identifier)]
    #[token(token::category::IDENTIFIER)]
    identifiers: ListField,

    #[composite(subcategory)]
    #[token(token::category::SUB)]
    subcategories: ListField,
}

/// Error type for issues creatin an [`IdentifierContainer`].
#[derive(Clone, Debug, Error)]
#[error("{0}")]
pub struct IdentifierErr(String);

impl std::convert::AsRef<Category> for Category {
    fn as_ref(&self) -> &Category {
        self
    }
}

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
    #[must_use]
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
        let mut identifier_container: IdentifierContainer<'a> = IdentifierContainer::default();

        for identifier in self.identifiers() {
            let ty = identifier.get(..1).ok_or_else(|| {
                IdentifierErr(format!(
                    "identifier \"{identifier}\" does not start with an ascii character"
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
                        "invalid identifier specifier '{spec}' in identifier: {identifier}"
                    )));
                }
            }
        }

        Ok(identifier_container)
    }

    /// Apply the category to a [`BufferStorage`] of bookmarks.
    ///
    /// # Errors
    /// If the Category cirteria are malformed.
    pub fn apply(&self, bookmarks: &mut BufferStorage<Bookmark>) -> Result<(), IdentifierErr> {
        let criteria = self.identifier_container()?;

        let include_matcher = aho_corasick::AhoCorasickBuilder::new()
            .ascii_case_insensitive(true)
            .auto_configure(&criteria.include)
            .build(&criteria.include);

        bookmarks.filter_in_place(|bookmark| {
            criteria.require.iter().all(|r| bookmark.url().contains(r))
                && (criteria.whole.iter().any(|w| *w == bookmark.url())
                    || include_matcher.is_match(bookmark.url()))
        });

        Ok(())
    }
}

impl Section for Category {
    const ITEM_NAME: &'static str = "category";
    const TOKEN_END: &'static str = token::category::END;
    const TOKEN_BEGIN: &'static str = token::category::BEGIN;
}
