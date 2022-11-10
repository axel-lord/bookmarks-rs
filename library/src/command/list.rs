use std::{cell::RefCell, ops::Range, rc::Rc};

use super::{buffer_length, get_bookmark_iter};

use crate::{
    bookmark::Bookmark,
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List {
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    buffer: Rc<RefCell<Vec<Range<usize>>>>,
}

impl Command for List {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let bookmarks = self.bookmarks.borrow();
        let buffer = self.buffer.borrow();

        // procedure to print bookmarks
        let print_bookmarks = |from, count| {
            get_bookmark_iter(&bookmarks, &buffer)
                .skip(from)
                .take(count)
                .for_each(|(_, b)| println!("{b}"))
        };

        // if one or more arguments get count else print all bookmarks and exit
        let [count, ..] = &args else {
            println!("listing all bookmarks");
            print_bookmarks(0, bookmarks.len());
            return Ok(());
        };

        // convert count from a string to an integer
        let Ok(count) = count.parse() else {
            return Err(CommandErr::Execution(format!(
                "could not parse {count} as a bookmark count"
            )));
        };

        // if two or more arguments get from (second argument) else print count bookmarks from
        // start and exit
        let [_, from, ..] = &args else {
            println!("listing {count} bookmarks");
            print_bookmarks(0, count);
            return Ok(());
        };

        // convert from from a string to an integer
        let Ok(from) = from.parse() else {
            return Err(CommandErr::Execution(format!(
                "could not parse {from} as a bookmark index"
            )));
        };

        // if from is negative wrap it to a positive based on buffer length
        let from = wrap_if_negative(from, buffer_length(&buffer))?;

        // if three or more arguments contiune else print count bookmarks from index from
        let [_, _, _, ..] = &args else {
            println!("listing {count} bookmarks starting at index {from}");
            print_bookmarks(from, count);
            return Ok(());
        };

        // since three or more arguments are not supported exit with a result indicating this
        Err(CommandErr::Execution(format!(
            "too many arguments passed to list ({})",
            args.len()
        )))
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
