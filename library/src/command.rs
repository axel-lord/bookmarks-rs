pub mod bookmark;
pub mod category;
pub mod list;
pub mod load;
pub mod reset;
pub mod save;

use std::{error::Error, ops::Range};

use crate::bookmark::Bookmark;

fn get_bookmark_iter<'a>(
    bookmarks: &'a Vec<Bookmark>,
    buffer: &'a Vec<Range<usize>>,
) -> impl Iterator<Item = (usize, &'a Bookmark)> {
    buffer
        .iter()
        .map(|r| r.clone().into_iter().map(|i| (i, &bookmarks[i])))
        .flatten()
}

fn buffer_length(buffer: &Vec<Range<usize>>) -> usize {
    buffer.iter().map(Range::len).fold(0, |acc, x| acc + x)
}

fn bookmark_filter_iter<'a, F>(
    bookmarks: impl 'a + Iterator<Item = (usize, &'a Bookmark)>,
    mut condition: F,
) -> impl Iterator<Item = Range<usize>> + 'a
where
    F: 'a + FnMut(&Bookmark) -> bool,
{
    bookmarks.filter_map(move |(i, bookmark)| {
        if condition(bookmark) {
            Some(i..i + 1)
        } else {
            None
        }
    })
}

fn get_filtered_bookmarks<'a, I, F>(bookmarks: I, condition: F) -> Vec<Range<usize>>
where
    I: 'a + Iterator<Item = (usize, &'a Bookmark)>,
    F: 'a + FnMut(&Bookmark) -> bool,
{
    bookmark_filter_iter(bookmarks, condition).collect()
}

pub fn command_debug(args: &[String]) -> Result<(), CommandErr> {
    println!("{:#?}", args);
    Ok(())
}

#[derive(Debug, Clone)]
pub enum CommandErr {
    Lookup,
    Execution(String),
}

impl std::fmt::Display for CommandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandErr::Lookup => write!(f, "command lookup failed"),
            CommandErr::Execution(ref msg) => write!(f, "command execution failed: {}", msg),
        }
    }
}

impl Error for CommandErr {}

impl From<bookmark_storage::ParseErr> for CommandErr {
    fn from(err: bookmark_storage::ParseErr) -> Self {
        Self::Execution(format!("{err}"))
    }
}

impl From<std::io::Error> for CommandErr {
    fn from(err: std::io::Error) -> Self {
        Self::Execution(format!("{err}"))
    }
}

pub trait Command {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr>;
}

impl<T> Command for T
where
    T: FnMut(&[String]) -> Result<(), CommandErr>,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self(args)
    }
}
