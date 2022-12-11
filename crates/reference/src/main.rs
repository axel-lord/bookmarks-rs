#[derive(Debug, Clone)]
pub struct Reference {
    line: bookmark_storage::ContentString,

    name: bookmark_storage::Field,

    children: bookmark_storage::ListField,

    info: bookmark_storage::Field,

    tags: bookmark_storage::ListField,
}

impl Reference {}

impl bookmark_storage::Storeable for Reference {
    fn with_string(
        line: String,
        line_num: Option<usize>,
    ) -> Result<Self, bookmark_storage::ParseErr> {
        let err = || bookmark_storage::ParseErr::Line(Some(line.clone()), line_num);
        let len = || line.len();

        use aho_corasick::AhoCorasick;
        use lazy_static::lazy_static;
        lazy_static! {
            static ref AC: AhoCorasick =
                AhoCorasick::new(["<name>", "<children>", "<info>", "<tags>"]);
        }

        let mut iter = AC.find_iter(&line).enumerate().peekable();

        // completely repeatable
        let (i, mat) = iter.next().ok_or_else(err)?;
        let start = mat.end();
        let end = iter.peek().map(|m| m.1.start()).unwrap_or_else(len);
        if start > end || mat.pattern() != i {
            return Err(err());
        }

        // unique for every match, single field pattern
        let name =
            bookmark_storage::pattern_match::substring_location(&line, line[start..end].trim())
                .ok_or_else(err)?
                .into();

        // completely repeatable
        let (i, mat) = iter.next().ok_or_else(err)?;
        let start = mat.end();
        let end = iter.peek().map(|m| m.1.start()).unwrap_or_else(len);
        if start > end || mat.pattern() != i {
            return Err(err());
        }

        // unique for every match, single field pattern
        let children = bookmark_storage::pattern_match::split_list_field(&line[start..end])
            .map(|f| f + start)
            .collect();

        // completely repeatable
        let (i, mat) = iter.next().ok_or_else(err)?;
        let start = mat.end();
        let end = iter.peek().map(|m| m.1.start()).unwrap_or_else(len);
        if start > end || mat.pattern() != i {
            return Err(err());
        }

        // unique for every match, single field pattern
        let info =
            bookmark_storage::pattern_match::substring_location(&line, line[start..end].trim())
                .ok_or_else(err)?
                .into();

        // completely repeatable
        let (i, mat) = iter.next().ok_or_else(err)?;
        let start = mat.end();
        let end = iter.peek().map(|m| m.1.start()).unwrap_or_else(len);
        if start > end || mat.pattern() != i {
            return Err(err());
        }

        // unique for every match, single field pattern
        let tags = bookmark_storage::pattern_match::split_list_field(&line[start..end])
            .map(|f| f + start)
            .collect();

        Ok(Self {
            line: line.into(),
            name,
            children,
            info,
            tags,
        })
    }

    fn to_line(&self) -> String {
        Self::create_line(self.name(), self.children(), self.info(), self.tags())
    }

    fn is_edited(&self) -> bool {
        self.line.is_appended_to()
    }

    fn get(
        &self,
        property: &str,
    ) -> Result<bookmark_storage::Property, bookmark_storage::PropertyErr> {
        Ok(match property {
            "name" => bookmark_storage::Property::Single(self.name().into()),
            "info" => bookmark_storage::Property::Single(self.info().into()),
            "children" => {
                bookmark_storage::Property::List(self.children().map(String::from).collect())
            }
            "tags" => bookmark_storage::Property::List(self.tags().map(String::from).collect()),
            _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
        })
    }

    fn set(
        &mut self,
        property: &str,
        value: bookmark_storage::Property,
    ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
        match (property, value) {
            ("name", bookmark_storage::Property::Single(value)) => self.set_name(&value),
            ("info", bookmark_storage::Property::Single(value)) => self.set_info(&value),
            ("children", bookmark_storage::Property::List(values)) => {
                self.set_children(values.iter())
            }
            ("tags", bookmark_storage::Property::List(values)) => self.set_tags(values.iter()),
            _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
        };
        Ok(self)
    }
    fn push(
        &mut self,
        property: &str,
        value: &str,
    ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
        match property {
            "children" => self.push_child(value),
            "tags" => self.push_tag(value),
            _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
        };
        Ok(self)
    }
}

impl Reference {
    pub fn create_line(
        name: &str,
        children: impl Iterator<Item = impl AsRef<str>>,
        info: &str,
        tags: impl Iterator<Item = impl AsRef<str>>,
    ) -> String {
        format!(
            "{} {} {} {} {} {} {} {}",
            "<name>",
            name,
            "<children>",
            bookmark_storage::pattern_match::join_with_delim(children),
            "<info>",
            info,
            "<tags>",
            bookmark_storage::pattern_match::join_with_delim(tags),
        )
    }

    pub fn new<'a>(
        name: &str,
        children: impl 'a + Iterator<Item = impl AsRef<str>>,
        info: &str,
        tags: impl 'a + Iterator<Item = impl AsRef<str>>,
    ) -> Self {
        let mut line = bookmark_storage::ContentString::new();
        Self {
            name: line.push(name).into(),
            children: line.extend(children).into(),
            info: line.push(info).into(),
            tags: line.extend(tags).into(),
            line,
        }
    }

    //
    // Name
    //

    pub fn name(&self) -> &str {
        self.name.get(&self.line)
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = self.line.push(name).into();

        self
    }

    //
    // Info
    //

    pub fn info(&self) -> &str {
        self.info.get(&self.line)
    }

    pub fn set_info(&mut self, info: &str) -> &mut Self {
        self.info = self.line.push(info).into();

        self
    }

    //
    // Children
    //

    pub fn children(&self) -> impl Iterator<Item = &str> {
        self.children.get(&self.line)
    }

    pub fn set_children(&mut self, children: impl Iterator<Item = impl AsRef<str>>) -> &mut Self {
        self.children.clear();

        for item in children {
            self.children.push(self.line.push(item.as_ref()).into());
        }

        self
    }

    pub fn push_child(&mut self, child: &str) -> &mut Self {
        self.children.push(self.line.push(child).into());

        self
    }

    //
    // Tags
    //

    pub fn tags(&self) -> impl Iterator<Item = &str> {
        self.tags.get(&self.line)
    }

    pub fn set_tags(&mut self, tags: impl Iterator<Item = impl AsRef<str>>) -> &mut Self {
        self.tags.clear();

        for item in tags {
            self.tags.push(self.line.push(item.as_ref()).into());
        }

        self
    }

    pub fn push_tag(&mut self, tag: &str) -> &mut Self {
        self.tags.push(self.line.push(tag).into());

        self
    }
}

impl From<Reference> for String {
    fn from(refr: Reference) -> Self {
        use bookmark_storage::Storeable;
        refr.to_line()
    }
}

impl std::convert::TryFrom<String> for Reference {
    type Error = bookmark_storage::ParseErr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        use bookmark_storage::Storeable;
        Self::with_string(value, None)
    }
}

impl std::convert::TryFrom<&str> for Reference {
    type Error = bookmark_storage::ParseErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use bookmark_storage::Storeable;
        Self::with_str(value, None)
    }
}

#[allow(clippy::write_literal)]
impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !f.alternate() {
            write!(f, "{}", "<name>")?;
            write!(f, " {} ", self.name())?;

            write!(f, "{}", "<children>")?;
            bookmark_storage::pattern_match::write_delim_list(f, self.children())?;

            write!(f, "{}", "<info>")?;
            write!(f, " {} ", self.info())?;

            write!(f, "{}", "<tags>")?;
            bookmark_storage::pattern_match::write_delim_list(f, self.children())?;
        } else {
            write!(f, "{}:", self.name())?;

            if !self.children.is_empty() {
                write!(f, "\n\tchildren: ")?;
                bookmark_storage::pattern_match::write_list_field(f, self.children())?;
            }

            write!(f, "\n\tinfo: {}", self.info())?;

            if !self.tags.is_empty() {
                write!(f, "\n\ttags: ")?;
                bookmark_storage::pattern_match::write_list_field(f, self.tags())?;
            }
        }
        Ok(())
    }
}

fn main() {
    use bookmark_storage::Storeable;
    let item = Reference::with_str(
        "<name> hello there <children> general <,> kenobi <info> blast them <tags> wow <,> nice",
        None,
    )
    .unwrap();
    dbg!(&item);
    dbg!(bookmark_storage::pattern_match::join_with_delim(
        ["hello", "there"].into_iter()
    ));
    dbg!(Reference::create_line(
        "Kenobi",
        ["hello", "there"].into_iter(),
        "general",
        ["nice"].into_iter()
    ));

    println!("{item}");
    println!("{:#}", item);
}
