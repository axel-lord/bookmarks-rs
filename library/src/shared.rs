mod buffer;
mod selected;
mod storage;

use bookmark_storage::Storeable;
pub use buffer::Buffer;
pub use selected::Selected;
pub use storage::{Bookmarks, Categroies, Infos, Storage};

#[derive(Debug, Default)]
pub struct BufferStorage<T>
where
    T: Storeable,
{
    pub storage: Storage<T>,
    pub buffer: Buffer,
    pub selected: Selected,
}

impl<T> Clone for BufferStorage<T>
where
    T: Storeable,
{
    fn clone(&self) -> Self {
        Self {
            storage: self.storage.clone(),
            buffer: self.buffer.clone(),
            selected: self.selected.clone(),
        }
    }
}

impl<T> BufferStorage<T>
where
    T: Storeable,
{
    pub fn new(storage: Storage<T>, buffer: Buffer, selected: Selected) -> Self {
        Self {
            storage,
            buffer,
            selected,
        }
    }

    pub fn reset(&self) -> &Self {
        self.buffer.reset();
        self.selected.clear();
        self
    }
}
