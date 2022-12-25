use std::sync::{Arc, Mutex};

use bookmark_library::Bookmark;

use crate::MainContent;

#[derive(Debug, Clone)]
pub enum Msg {
    GotoBookmarkLocation(usize),
    ApplyCategory(Box<[usize]>),
    UpdateShownBookmarks(String),
    UpdateShownFrom(String),
    UpdateShownFromSteps(isize),
    UpdateUrlWidth(String),
    UpdateDescWidth(String),
    FilterBookmarks(String),
    SwitchMainTo(MainContent),
    AddBookmarks(Arc<Mutex<Option<Vec<Bookmark>>>>),
    ApplyFilter,
    Reset,
    Tick,
}
