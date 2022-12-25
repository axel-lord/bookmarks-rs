use std::sync::{Arc, Mutex};

use bookmark_library::Bookmark;

use crate::MainContent;

#[derive(Debug, Clone)]
pub enum Msg {
    GotoBookmarkLocation(usize),
    ApplyCategory(Box<[usize]>),
    UpdateShownBookmarks(Box<str>),
    UpdateShownFrom(Box<str>),
    UpdateShownFromSteps(isize),
    UpdateUrlWidth(Box<str>),
    UpdateDescWidth(Box<str>),
    FilterBookmarks(Box<str>),
    SwitchMainTo(MainContent),
    AddBookmarks(Arc<Mutex<Option<Vec<Bookmark>>>>),
    ApplyFilter,
    Reset,
    Tick,
}
