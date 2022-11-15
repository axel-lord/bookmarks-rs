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
