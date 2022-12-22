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
    ApplyFilter,
    Reset,
    Tick,
}
