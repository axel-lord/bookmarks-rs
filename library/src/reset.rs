use crate::shared;

#[derive(Debug, Clone)]
pub struct ResetValues {
    pub bookmark_buffer: shared::Buffer,
    pub bookmarks: shared::Bookmarks,
    pub selected_bookmark: shared::Selected,
    pub selected_category: shared::Selected,
}

impl ResetValues {
    pub fn reset(&self) {
        self.bookmark_buffer.reset(&self.bookmarks);
        self.selected_bookmark.clear();
        self.selected_category.clear();
    }
}
