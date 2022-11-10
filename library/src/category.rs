use crate::{pattern_match, token, ContentString};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, error::Error, ops::Range};

#[derive(Debug)]
pub struct Category {
    line: Option<ContentString>,
    id: Range<usize>,
    name: Range<usize>,
    description: Range<usize>,
    identifier: Range<usize>,
    identifiers: Vec<Range<usize>>,
    subcategory: Range<usize>,
    subcategories: Vec<Range<usize>>,
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

#[derive(Clone, Debug)]
pub struct IdentifierErr(String);

impl std::fmt::Display for IdentifierErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IdentifierErr {}

impl Clone for Category {
    fn clone(&self) -> Self {
        Self::with_str(self.to_line(), None).unwrap()
    }
}

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
    pub fn new<'a>(
        id: &str,
        name: &str,
        description: &str,
        identifiers: impl Iterator<Item = &'a str>,
        subcategories: impl Iterator<Item = &'a str>,
    ) -> Self {
        Self::with_str(
            Self::create_line(id, name, description, identifiers, subcategories),
            None,
        )
        .unwrap()
    }

    pub fn with_str(line: String, line_num: Option<usize>) -> Result<Self, CategoryErr> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(
                &[
                    r"^",
                    token::category::ID,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::category::NAME,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::category::DESCRIPTION,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::category::IDENTIFIER,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    token::category::SUBCATEGORY,
                    pattern_match::WHITESPACE_PADDED_GROUP,
                    r"$"
                ]
                .concat()
            )
            .unwrap();
        }

        let err = || CategoryErr::LineParseFailure(line.clone(), line_num);

        let captures = LINE_RE.captures(&line).ok_or_else(err)?;

        let id = captures.get(1).ok_or_else(err)?.range();
        let name = captures.get(2).ok_or_else(err)?.range();
        let description = captures.get(3).ok_or_else(err)?.range();

        let identifier = captures.get(4).ok_or_else(err)?.range();
        let identifiers = pattern_match::split_by_delim_to_ranges(&line[identifier.clone()]);

        let subcategory = captures.get(5).ok_or_else(err)?.range();
        let subcategories = pattern_match::split_by_delim_to_ranges(&line[subcategory.clone()]);

        Ok(Self {
            line: Some(ContentString::UnappendedTo(line)),
            id,
            name,
            description,
            identifier,
            identifiers,
            subcategory,
            subcategories,
        })
    }

    pub fn to_line(&self) -> String {
        if let Some(ContentString::UnappendedTo(line)) = self.line.as_ref() {
            line.clone()
        } else {
            Self::create_line(
                self.id(),
                self.name(),
                self.description(),
                self.identifiers(),
                self.subcategories(),
            )
        }
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

    pub fn identifiers(&self) -> impl Iterator<Item = &str> {
        self.identifiers
            .iter()
            .map(|r| &self.raw_line()[self.identifier.clone()][r.clone()])
    }

    pub fn subcategories(&self) -> impl Iterator<Item = &str> {
        self.subcategories
            .iter()
            .map(|r| &self.raw_line()[self.subcategory.clone()][r.clone()])
    }

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

    pub fn subcategory_vec<'a>(
        &self,
        categories: impl Clone + Iterator<Item = &'a Category>,
    ) -> Vec<&'a Category> {
        self.subcategories()
            .filter_map(|subcat| categories.clone().find(|cat| cat.id() == subcat))
            .collect()
    }

    fn create_line<'a>(
        id: &str,
        name: &str,
        description: &str,
        identifiers: impl Iterator<Item = &'a str>,
        subcategories: impl Iterator<Item = &'a str>,
    ) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {}",
            token::category::ID,
            id,
            token::category::NAME,
            name,
            token::category::DESCRIPTION,
            description,
            token::category::IDENTIFIER,
            identifiers
                .collect::<Vec<&str>>()
                .join(&[" ", token::DELIM, " "].concat()),
            token::category::SUBCATEGORY,
            subcategories
                .collect::<Vec<&str>>()
                .join(&[" ", token::DELIM, " "].concat()),
        )
    }

    fn raw_line(&self) -> &str {
        self.line.as_ref().unwrap().ref_any()
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