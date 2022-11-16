use bookmark_storage::{
    content_string::ContentString, Field, ListField, ParseErr, Property, PropertyErr, Storeable,
};

#[derive(Debug)]
pub struct Reference {
    line: ContentString,

    name: Field,

    children: ListField,
}

impl Reference {}

impl Storeable for Reference {
    fn with_string(line: String, line_num: Option<usize>) -> Result<Self, ParseErr> {
        use lazy_static::lazy_static;
        lazy_static! {
            static ref LINE_RE: regex::Regex = regex::Regex::new(
                &[
                    "^",
                    "<name>",
                    bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
                    "<children>",
                    bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
                    "$",
                ]
                .concat()
            )
            .unwrap();
        }

        let err = || bookmark_storage::ParseErr::Line(Some(line.clone()), line_num);
        let captures = LINE_RE.captures(&line).ok_or_else(err)?;

        let name = captures.get(1).ok_or_else(err)?.range().into();

        let group = captures.get(2).ok_or_else(err)?.range();
        let mut children: ListField = bookmark_storage::pattern_match::split_by_delim_to_ranges(
            line.get(group.clone()).unwrap(),
        )
        .into();
        children.iter_mut().for_each(|item| {
            *item += group.start;
        });

        Ok(Self {
            line: line.into(),
            name,
            children,
        })
    }

    fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, ParseErr> {
        Self::with_string(line.into(), line_num)
    }

    fn to_line(&self) -> String {
        std::unimplemented!()
    }

    fn is_edited(&self) -> bool {
        self.line.is_appended_to()
    }

    fn get(&self, property: &str) -> Result<Property, PropertyErr> {
        std::unimplemented!()
    }

    fn set(&mut self, property: &str, value: Property) -> Result<(), PropertyErr> {
        std::unimplemented!()
    }
    fn push(&mut self, property: &str, value: &str) -> Result<(), PropertyErr> {
        std::unimplemented!()
    }
}

fn main() {
    let item =
        Reference::with_str("<name> hello there <children> general <,> kenobi", None).unwrap();
    dbg!(item);
}
