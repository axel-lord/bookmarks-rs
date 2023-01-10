use aho_corasick::AhoCorasick;
use bookmark_library::{container, Bookmark, Category, Info};
use iced::widget;

use crate::MainContent;

use super::Metrics;

/// View of the application state, providing easy immutable read in building of view.
#[derive(Clone, Copy, Debug)]
pub struct View<'a> {
    /// When bookmarks scrollbar is created will be set to it's id.
    pub bookmark_scrollbar_id: &'a widget::scrollable::Id,
    /// Bookmarks loaded by application.
    pub bookmarks: &'a container::BufferStorage<Bookmark>,
    /// Categories loaded by application.
    pub categories: &'a container::BufferStorage<Category>,
    /// Category indices arranged with parents at start.
    pub category_tree: &'a [Vec<usize>],
    /// Expected max character count of bookmark descriptions deisplayed as numeric and str.
    pub desc_width: (usize, &'a str),
    /// Is true if edit mode is enabled.
    pub edit_mode_active: bool,
    /// Filter used for bookmarks as filter object and str.
    pub filter: (Option<&'a AhoCorasick>, &'a str),
    /// Info loaded by application.
    pub infos: &'a container::BufferStorage<Info>,
    /// True if dark mode in use.
    pub is_dark_mode: bool,
    /// What is expected to fill the main area.
    pub main_content: MainContent,
    /// Stats that may have been gathered.
    pub metrics: &'a Metrics,
    /// How many bookmarks are shown as numeric and str.
    pub shown_bookmarks: (usize, &'a str),
    /// Where in the bookmark list to start showing bookmarks as numeric and str.
    pub shown_from: (usize, &'a str),
    /// Current status message.
    pub status: &'a str,
    /// All status messages.
    pub status_log: &'a [String],
    /// Expected max cahgracter count of bookmark urls displayed as numeric and str.
    pub url_width: (usize, &'a str),
}
