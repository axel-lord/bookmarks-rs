use crate::{bookmark::Bookmark, category::Category};
use bookmark_storage::Storeable;
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug)]
pub struct Storage<T: Storeable>(Rc<RefCell<Vec<T>>>);

impl<T> Storage<T>
where
    T: Storeable,
{
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }

    pub fn borrow(&self) -> Ref<Vec<T>> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<Vec<T>> {
        self.0.borrow_mut()
    }
}

impl<T> Clone for Storage<T>
where
    T: Storeable,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Storage<T>
where
    T: Storeable,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type Bookmarks = Storage<Bookmark>;

pub type Categroies = Storage<Category>;
