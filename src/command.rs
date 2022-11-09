pub mod count;
pub mod filter;
pub mod list;
pub mod load;
pub mod regex;
pub mod reset;
pub mod save;

pub use self::count::Count;
pub use self::filter::{Filter, FilterInv};
pub use self::list::List;
pub use self::load::Load;
pub use self::regex::{Regex, RegexInv};
pub use self::reset::Reset;
pub use self::save::Save;

use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::command_map::CommandErr;
use crate::{bookmark::Bookmark, command_map::CommandMap};

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

pub fn build_command_map(bookmarks: Rc<RefCell<Vec<Bookmark>>>) -> CommandMap<'static> {
    let mut command_map = CommandMap::new();
    let buffer = Rc::new(RefCell::new(vec![(0..bookmarks.borrow().len())]));

    command_map.push("list", List::build(bookmarks.clone(), buffer.clone()));

    command_map.push("filter", Filter::build(bookmarks.clone(), buffer.clone()));

    command_map.push(
        "filter-inv",
        FilterInv::build(bookmarks.clone(), buffer.clone()),
    );

    command_map.push("regex", Regex::build(bookmarks.clone(), buffer.clone()));

    command_map.push(
        "regex-inv",
        RegexInv::build(bookmarks.clone(), buffer.clone()),
    );

    command_map.push("count", Count::build(bookmarks.clone(), buffer.clone()));

    command_map.push("reset", Reset::build(bookmarks.clone(), buffer.clone()));

    command_map.push("load", Load::build(bookmarks.clone()));

    command_map.push("save", Save::build(bookmarks.clone(), buffer.clone()));

    command_map.push("debug", Box::new(command_debug));

    command_map
}
