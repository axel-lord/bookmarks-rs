mod buffer;
mod selected;
mod storage;

use bookmark_storage::Storeable;
pub use buffer::Buffer;
pub use selected::Selected;
pub use storage::{Bookmarks, Categroies, Infos, Storage};

#[derive(Debug)]
pub struct BufferStorage<T>(pub Storage<T>, pub Buffer, pub Selected)
where
    T: Storeable;

impl<T> Default for BufferStorage<T>
where
    T: Storeable,
{
    fn default() -> Self {
        BufferStorage::<T>(Default::default(), Default::default(), Default::default())
    }
}

impl<T> Clone for BufferStorage<T>
where
    T: Storeable,
{
    fn clone(&self) -> Self {
        BufferStorage::<T>(self.0.clone(), self.1.clone(), self.2.clone())
    }
}
