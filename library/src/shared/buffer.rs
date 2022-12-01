use bookmark_storage::Storeable;

use std::sync::{Arc, RwLock};

use super::Storage;

#[derive(Clone, Debug, Default)]
pub struct Buffer(Arc<RwLock<Option<Vec<usize>>>>);

impl Buffer {
    pub fn count(&self) -> Option<usize> {
        Some(self.0.read().unwrap().as_ref()?.len())
    }

    pub fn reset(&self) {
        self.0.write().unwrap().take();
    }

    pub fn replace(&self, with: Option<Vec<usize>>) {
        *self.0.write().unwrap() = with;
    }

    pub fn filter_in_place<F, T>(&self, content: &Storage<T>, mut f: F) -> &Self
    where
        F: FnMut(&T) -> bool,
        T: Storeable,
    {
        let mut internal = self.0.write().unwrap();

        let current = internal.take();
        let content = content.read();

        internal.replace(if let Some(current) = current {
            current.into_iter().filter(|i| f(&content[*i])).collect()
        } else {
            content
                .iter()
                .enumerate()
                .filter(|(_, v)| f(v))
                .map(|(i, _)| i)
                .collect()
        });
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        if self.0.read().unwrap().is_some() {
            EitherIter::A(self.0.read().unwrap().as_ref().unwrap().clone().into_iter())
        } else {
            EitherIter::B(0..)
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
