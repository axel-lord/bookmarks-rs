#[derive(Debug, Clone)]
pub enum Msg {
    GotoBookmarkLocation(usize),
    ApplyCategory(usize),
    UpdateShownBookmarks(Box<str>),
    UpdateShownFrom(Box<str>),
    UpdateShownFromSteps(isize),
    UpdateUrlWidth(Box<str>),
    UpdateDescWidth(Box<str>),
    FilterBookmarks(Box<str>),
    ApplyFilter,
    Reset,
}
