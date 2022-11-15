use std::{
    cell::RefCell,
    ops::{Deref, Range},
    rc::Rc,
};

use crate::{bookmark::Bookmark, category::Category};

#[derive(Debug)]
pub struct Storage<T>(Rc<RefCell<Vec<T>>>);

impl<T> Deref for Storage<T> {
    type Target = RefCell<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Storage<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type Bookmarks = Storage<Bookmark>;
pub type Categroies = Storage<Category>;

impl Bookmarks {
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
}

impl Buffer {
    pub fn bookmark_count(&self) -> usize {
        self.borrow().iter().map(Range::len).sum()
    }

    pub fn bookmarks(&self, bookmarks: &Vec<Bookmark>) -> Vec<Bookmark> {
        Self::bookmark_iter(&self.borrow(), bookmarks)
            .map(|(_, bookmark)| bookmark.clone())
            .collect()
    }

    pub fn bookmark_iter<'a>(
        buffer: &'a Vec<Range<usize>>,
        bookmarks: &'a Vec<Bookmark>,
    ) -> impl Iterator<Item = (usize, &'a Bookmark)> {
        buffer
            .iter()
            .map(|r| r.clone().into_iter().map(|i| (i, &bookmarks[i])))
            .flatten()
    }

    pub fn unenumerated_bookmark_iter<'a>(
        buffer: &'a Vec<Range<usize>>,
        bookmarks: &'a Vec<Bookmark>,
    ) -> impl Iterator<Item = &'a Bookmark> {
        buffer
            .iter()
            .map(|r| r.clone().into_iter().map(|i| &bookmarks[i]))
            .flatten()
    }

    pub fn filter<F>(&self, bookmarks: &Vec<Bookmark>, condition: F) -> Vec<Range<usize>>
    where
        F: Fn(&Bookmark) -> bool,
    {
        Self::bookmark_iter(&self.borrow(), bookmarks)
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
shared!(Selected, Option<usize>);
