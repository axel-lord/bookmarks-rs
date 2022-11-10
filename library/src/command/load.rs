use std::{cell::RefCell, fs::File, io, rc::Rc};

use crate::{
    bookmark::{Bookmark, BookmarkErr},
    command_map::{Command, CommandErr},
    token,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
}

impl Command for Load {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let file = File::open(&args[0]).map_err(|err| {
            CommandErr::Execution(format!("could not open {}: {}", &args[0], err))
        })?;

        let content = io::read_to_string(file).map_err(|err| {
            CommandErr::Execution(format!("failed to read {}: {}", &args[0], err))
        })?;

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
                    "could not parse line {} of {}: {}",
                    i, &args[0], line
                )));
            }
            Err(BookmarkErr::LineParseFailure(line, None)) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse a line of {}: {}",
                    &args[0], line
                )));
            }
        };

        self.bookmarks.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
