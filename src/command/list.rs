use std::{cell::RefCell, ops::Range, rc::Rc};

use super::{buffer_length, get_bookmark_iter};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

pub struct List {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl List {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        Box::new(Self { bookmarks, buffer })
    }
}

impl Command for List {
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr> {
        let bookmarks = self.bookmarks.borrow();
        let buffer = self.buffer.borrow();

        let bookmark_iter = get_bookmark_iter(&bookmarks, &buffer);
        match &args[..] {
            [] => {
                println!("listing all bookmarks");
                for (_, bookmark) in bookmark_iter {
                    println!("{bookmark}");
                }
                Ok(())
            }
            [count] => {
                println!("listing {count} bookmarks");
                let count = match count.parse() {
                    Ok(c) => c,
                    Err(_) => {
                        return Err(CommandErr::Execution(format!(
                            "could not parse {count} as a bookmark count"
                        )))
                    }
                };
                for (_, bookmark) in bookmark_iter.take(count) {
                    println!("{bookmark}");
                }
                Ok(())
            }
            [count, from] => {
                println!("listing {count} bookmarks starting at index {from}");
                let count = match count.parse() {
                    Ok(c) => c,
                    Err(_) => {
                        return Err(CommandErr::Execution(format!(
                            "could not parse {count} as a bookmark count"
                        )))
                    }
                };
                let from = match from.parse() {
                    Ok(f) => f,
                    Err(_) => {
                        return Err(CommandErr::Execution(format!(
                            "could not parse {count} as a bookmark index"
                        )))
                    }
                };

                let from = wrap_if_negative(from, buffer_length(&buffer))?;

                for (_, bookmark) in bookmark_iter.skip(from).take(count) {
                    println!("{bookmark}");
                }
                Ok(())
            }
            _ => Err(CommandErr::Execution("Usage: list [COUNT] [FROM]".into())),
        }
    }
}

fn wrap_if_negative(number: isize, max: usize) -> Result<usize, CommandErr> {
    if number.abs() as usize > max {
        return Err(CommandErr::Execution(format!(
            "number {number} larger than max value {max}"
        )));
    }

    Ok(if number >= 0 {
        number as usize
    } else {
        max - number.abs() as usize
    })
}
