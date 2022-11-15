use std::{
    cell::RefCell,
    ops::{Deref, Range},
    rc::Rc,
};

use crate::{bookmark::Bookmark, category::Category};

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

shared!(Bookmarks, Vec<Bookmark>);
shared!(Categroies, Vec<Category>);
shared!(Buffer, Vec<Range<usize>>);
shared!(Selected, Option<usize>);
