pub mod bookmark;
pub mod category;

pub mod reset;

use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command_map::{CommandErr, CommandMap},
};

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

fn command_debug(args: &[String]) -> Result<(), CommandErr> {
    println!("{:#?}", args);
    Ok(())
}

pub fn build_command_map(
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
    categories: Rc<RefCell<Vec<Category>>>,
) -> CommandMap<'static> {
    let mut command_map = CommandMap::new();
    let buffer = Rc::new(RefCell::new(vec![(0..bookmarks.borrow().len())]));

    command_map.push(
        "reset",
        None,
        reset::Reset::build(bookmarks.clone(), buffer.clone()),
    );

    command_map.push(
        "category",
        None,
        category::Category::build(categories.clone()),
    );

    command_map.push(
        "bookmark",
        None,
        bookmark::Bookmark::build(bookmarks.clone(), buffer.clone()),
    );

    command_map.push("debug", None, Box::new(command_debug));

    command_map
}
