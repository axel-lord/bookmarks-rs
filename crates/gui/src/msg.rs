use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use bookmark_library::Bookmark;
use iced::{
    widget::pane_grid::{DragEvent, Pane, ResizeEvent},
    Theme,
};

use crate::{
    app::{ui::edit_column, Metric},
    MainContent,
};

/// Message type for gui.
#[derive(Debug, Clone, Default)]
pub enum Msg {
    /// Open bookmark at passed index.
    GotoBookmarkLocation(usize),
    /// Apply all categories with passed indices.
    ApplyCategory(Vec<usize>),
    /// Update amount of bookmarks shown.
    UpdateShownBookmarks(String),
    /// Update start of shown bookmarks.
    UpdateShownFrom(String),
    /// Move the passed nomber of pages of bookmarks.
    UpdateShownFromSteps(isize),
    /// Update url max width.
    UpdateUrlWidth(String),
    /// Update Description max width.
    UpdateDescWidth(String),
    /// Build active bookmark filter with given string.
    FilterBookmarks(String),
    /// Switch the content of the main area.
    SwitchMainTo(MainContent),
    /// Add passed bookmarks.
    AddBookmarks(Arc<Mutex<Option<Vec<Bookmark>>>>),
    /// Enable or Disable edit mode.
    SetEditMode(bool),
    /// Bookmark at passed index should be edited.
    EditBookmark(usize),
    /// Catgegory at passed index should be edited.
    EditCategory(usize),
    /// Message for edit column.
    EditColumnMessage(edit_column::Message),
    /// Used for when resizing log panes.
    LogPaneResize(ResizeEvent),
    /// Close a pane in the log section.
    CloseLogPane(Pane),
    /// A pane was dragged in the log section.
    DragLogPane(DragEvent),
    /// Signal that some stats are to be gathered.
    GatherMetric(Metric),
    /// Set the theme in use.
    SetTheme(Theme),
    /// Don't use some generated value but log it.
    Debug(Arc<dyn Debug + Send + Sync>),
    /// The filter in the filter box should be filter the bookmarks until reset.
    ApplyFilter,
    /// Any and all bookmark filters should be removed.
    Reset,
    /// Misc. checks and updates should be performed.
    Tick,
    /// When a message needs to be sent but nothing should be done.
    #[default]
    None,
}
