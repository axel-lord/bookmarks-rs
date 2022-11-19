use bookmark_storage::Storeable;

use std::{cell::RefCell, rc::Rc};

use super::Storage;

#[derive(Clone, Debug, Default)]
pub struct Buffer(Rc<RefCell<Option<Vec<usize>>>>);

impl Buffer {
    pub fn count(&self) -> Option<usize> {
        Some(self.0.borrow().as_ref()?.len())
    }

    pub fn reset(&self) {
        self.0.borrow_mut().take();
    }

    pub fn filter_in_place<F, T>(&self, content: &Storage<T>, mut f: F) -> &Self
    where
        F: FnMut(&T) -> bool,
        T: Storeable,
    {
        let current = self.0.borrow_mut().take();
        let content = content.borrow();
        if let Some(current) = current {
            self.0.replace(Some(
                current
                    .into_iter()
                    .filter(|i| f(&content[i.clone()]))
                    .collect(),
            ));
        }
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        if self.0.borrow().is_some() {
            EitherIter::A(self.0.borrow().as_ref().unwrap().clone().into_iter())
        } else {
            EitherIter::B((0..).into_iter())
        }
    }
}

enum EitherIter<A, B, T>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    A(A),
    B(B),
}

impl<A, B, T> Iterator for EitherIter<A, B, T>
where
    A: Iterator<Item = T>,
    B: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::A(ref mut i) => i.next(),
            Self::B(ref mut i) => i.next(),
        }
    }
}
