use crate::container;
use bookmark_storage::Storeable;
use std::sync::{Arc, RwLock};

pub type Buffer = Arc<RwLock<container::Buffer>>;
pub type Selected = Arc<RwLock<container::Selected>>;
pub type Storage<T> = Arc<RwLock<container::Storage<T>>>;

#[derive(Debug, Default)]
pub struct BufferStorage<T> {
    pub storage: Storage<T>,
    pub buffer: Buffer,
    pub selected: Selected,
}

impl<T> Clone for BufferStorage<T> {
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
        self.buffer.write().unwrap().reset();
        self.selected.write().unwrap().clear();
        self
    }
}
