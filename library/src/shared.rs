use std::{
    cell::RefCell,
    ops::{Deref, Range},
    rc::Rc,
};

use bookmark_storage::Storeable;

use crate::{bookmark::Bookmark, category::Category};

#[derive(Debug)]
pub struct Storage<T: Storeable>(Rc<RefCell<Vec<T>>>);

impl<T> Deref for Storage<T>
where
    T: Storeable,
{
    type Target = RefCell<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
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

impl Bookmarks {
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
}

pub type Categroies = Storage<Category>;

macro_rules! shared {
    ($name:ident, $content:ty) => {
        #[derive(Debug, Default)]
        pub struct $name(Rc<RefCell<$content>>);
        impl Deref for $name {
            type Target = RefCell<$content>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
    };
}

shared!(Buffer, Vec<Range<usize>>);

impl Buffer {
    pub fn bookmark_count(&self) -> usize {
        self.borrow().iter().map(Range::len).sum()
    }

    pub fn reset<T>(&self, items: &Storage<T>)
    where
        T: Storeable,
    {
        self.borrow_mut().clear();
        self.borrow_mut().push(0..items.borrow().len());
    }

    pub fn enumerated_iter<'a, T>(
        buffer: &'a Vec<Range<usize>>,
        items: &'a Vec<T>,
    ) -> impl Iterator<Item = (usize, &'a T)>
    where
        T: Storeable,
    {
        buffer
            .iter()
            .map(|r| r.clone().into_iter().map(|i| (i, &items[i])))
            .flatten()
    }

    pub fn unenumerated_iter<'a, T>(
        buffer: &'a Vec<Range<usize>>,
        items: &'a Vec<T>,
    ) -> impl Iterator<Item = &'a T>
    where
        T: Storeable,
    {
        buffer
            .iter()
            .map(|r| r.clone().into_iter().map(|i| &items[i]))
            .flatten()
    }

    pub fn filter<F>(&self, bookmarks: &Vec<Bookmark>, condition: F) -> Vec<Range<usize>>
    where
        F: Fn(&Bookmark) -> bool,
    {
        Self::enumerated_iter(&self.borrow(), bookmarks)
            .filter_map(move |(i, bookmark)| {
                if condition(bookmark) {
                    Some(i..i + 1)
                } else {
                    None
                }
            })
            .collect()
    }
}

shared!(Selected, Option<usize>);

impl Selected {
    pub fn get<'a, T>(&self, container: &'a Vec<T>) -> Option<&'a T> {
        container.get(self.borrow().clone()?)
    }

    pub fn get_mut<'a, T>(&self, container: &'a mut Vec<T>) -> Option<&'a mut T> {
        container.get_mut(self.borrow().clone()?)
    }

    pub fn is_empty(&self) -> bool {
        self.borrow().is_none()
    }

    pub fn clear(&self) {
        self.borrow_mut().take();
    }

    pub fn replace(&self, value: usize) {
        self.borrow_mut().replace(value);
    }
}
