use std::{
    cell::RefCell,
    ops::{Deref, Range},
    rc::Rc,
};

use crate::{bookmark::Bookmark, category::Category};

macro_rules! shared {
    ($name:ident, $content:ty) => {
        #[derive(Clone, Debug, Default)]
        pub struct $name(Rc<RefCell<$content>>);
        impl Deref for $name {
            type Target = RefCell<$content>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

shared!(Bookmarks, Vec<Bookmark>);
shared!(Categroies, Vec<Category>);
shared!(Buffer, Vec<Range<usize>>);
shared!(Selected, Option<usize>);
