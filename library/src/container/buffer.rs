/// A list of indices usefult when applying filters to some index based storage.
#[derive(Debug, Clone, Default)]
pub struct Buffer {
    indices: Option<Vec<usize>>,
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

impl Buffer {
    /// If a fixed amount of indices are in the buffer returns that amount, if a non fixed amount of
    /// indices are in the buffer, such as all indices, returns none.
    pub fn count(&self) -> Option<usize> {
        self.indices.as_ref().map(Vec::len)
    }

    /// Filters the indices in place by applying a condition to the objects the buffer represents.
    pub fn filter_in_place<T>(&mut self, content: &[T], mut f: impl FnMut(&T) -> bool) -> &Self {
        let filtered = self
            .indices
            .take()
            .map(|v| v.into_iter().filter(|i| f(&content[*i])).collect())
            .unwrap_or_else(|| {
                content
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| f(v))
                    .map(|(i, _)| i)
                    .collect()
            });

        self.indices.replace(filtered);

        self
    }

    /// Reset the buffer to a state representing all items being selected.
    pub fn reset(&mut self) -> &mut Self {
        self.indices.take();
        self
    }

    /// Get an iterator of the indices, if no fixed amount of indices exist the iterator will start
    /// at 0 and increase indefinately.
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.indices
            .as_ref()
            .map(|v| EitherIter::A(v.clone().into_iter()))
            .unwrap_or_else(|| EitherIter::B(0..))
    }
}
