use crate::{bookmark::Bookmark, category::Category, info::Info};
use bookmark_storage::Storeable;

use std::{
    cmp::Ordering,
    sync::{Arc, RwLock, RwLockReadGuard},
};

#[derive(Debug)]
pub struct Storage<T>(Arc<RwLock<Vec<T>>>)
where
    T: Storeable;

impl<T> Storage<T>
where
    T: Storeable,
{
    pub fn len(&self) -> usize {
        self.0.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.read().unwrap().is_empty()
    }

    pub fn push(&self, value: T) -> &Self {
        self.0.write().unwrap().push(value);
        self
    }

    pub fn read(&self) -> RwLockReadGuard<Vec<T>> {
        self.0.read().unwrap()
    }

    pub fn sort_by(&self, compare: impl FnMut(&T, &T) -> Ordering) -> &Self {
        self.0.write().unwrap().sort_by(compare);
        self
    }

    pub fn dedup_by(&self, same_bucket: impl FnMut(&mut T, &mut T) -> bool) -> &Self {
        self.0.write().unwrap().dedup_by(same_bucket);
        self
    }

    pub fn extend(&self, iter: impl Iterator<Item = T>) -> &Self {
        self.0.write().unwrap().extend(iter);
        self
    }

    // pub fn borrow(&self) -> Ref<Vec<T>> {
    //     self.0.borrow()
    // }

    // pub fn borrow_mut(&self) -> RefMut<Vec<T>> {
    //     self.0.borrow_mut()
    // }
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

pub type Infos = Storage<Info>;
