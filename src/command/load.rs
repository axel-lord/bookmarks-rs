use std::{cell::RefCell, collections::HashMap, fs::File, io, rc::Rc};

use crate::{
    bookmark::{Bookmark, BookmarkErr},
    category::{Category, CategoryErr},
    command_map::{Command, CommandErr},
    token,
};

pub struct Load {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
}

impl Load {
    pub fn build(bookmarks: Rc<RefCell<Vec<Bookmark>>>) -> Box<Self> {
        Box::new(Self { bookmarks })
    }
}

impl Command for Load {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let Ok(file) = File::open(&args[0]) else {
            return Err(CommandErr::Execution(format!("could not open {}", &args[0])));
        };

        let Ok(content) = io::read_to_string(file) else {
            return Err(CommandErr::Execution(format!("failed to read {}", &args[0])));
        };

        let category_iter = content
            .lines()
            .enumerate()
            .skip_while(|(_, l)| !l.contains(token::CATEGORY_BEGIN))
            .skip(1)
            .take_while(|(_, l)| !l.contains(token::CATEGORY_END))
            .map(|(i, l)| Category::with_str(l.into(), Some(i)));

        let categories = match category_iter.collect::<Result<Vec<Category>, CategoryErr>>() {
            Ok(categories) => categories,
            Err(CategoryErr::LineParseFailure(line, Some(i))) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse category on line {}: {}",
                    i, line
                )))
            }
            Err(CategoryErr::LineParseFailure(line, None)) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse category from line: {}",
                    line
                )))
            }
        };

        let identifier_count = {
            let mut map = HashMap::new();

            for identifier in categories
                .iter()
                .map(|c| c.identifiers())
                .flatten()
                .filter_map(|ident| ident.chars().next())
            {
                let entry = map.entry(identifier).or_insert(0usize);
                *entry += 1;
            }

            map
        };

        println!("{:#?}", identifier_count);

        let bookmark_iter = content
            .lines()
            .enumerate()
            .skip_while(|(_, l)| !l.contains(token::UNSORTED_BEGIN))
            .skip(1)
            .take_while(|(_, l)| !l.contains(token::UNSORTED_END))
            .map(|(i, l)| Bookmark::with_str(l.into(), Some(i)));

        let loaded = match bookmark_iter.collect::<Result<Vec<_>, _>>() {
            Ok(loaded) => {
                if loaded.is_empty() {
                    return Err(CommandErr::Execution(format!(
                        "could not parse any bookmarks from {}",
                        &args[0]
                    )));
                } else {
                    loaded
                }
            }
            Err(BookmarkErr::LineParseFailure(line, Some(i))) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse line {} of {}\n{}",
                    i, &args[0], line
                )));
            }
            Err(BookmarkErr::LineParseFailure(line, None)) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse a line of {}\n{}",
                    &args[0], line
                )));
            }
        };

        self.bookmarks.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
