use std::ops::Range;

use crate::bookmark::Bookmark;

pub fn reset(
    buffer: &mut Vec<Range<usize>>,
    bookmarks: &Vec<Bookmark>,
    selected_bookmark: &mut Option<usize>,
) {
    buffer.clear();
    buffer.push(0..bookmarks.len());
    selected_bookmark.take();
}
