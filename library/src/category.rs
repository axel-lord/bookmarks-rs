use bookmark_storage::{token, ContentString, Field, ListField, Section, Storeable};
use std::{collections::HashMap, error::Error};

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
    const ITEM_NAME: &'static str = "category";
    const TOKEN_END: &'static str = token::CATEGORY_END;
    const TOKEN_BEGIN: &'static str = token::CATEGORY_BEGIN;
}
