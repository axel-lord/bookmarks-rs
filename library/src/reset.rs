use crate::shared;

#[derive(Debug, Clone)]
pub struct ResetValues {
    pub bookmark_buffer: shared::Buffer,
    pub bookmarks: shared::Bookmarks,
    pub category_buffer: shared::Buffer,
    pub categories: shared::Categroies,
    pub selected_bookmark: shared::Selected,
    pub selected_category: shared::Selected,
}

impl ResetValues {
    pub fn reset(&self) {
        self.bookmark_buffer.reset(&self.bookmarks);
        self.category_buffer.reset(&self.categories);
        self.selected_bookmark.clear();
        self.selected_category.clear();
    }
}
