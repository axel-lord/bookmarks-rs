/// A list of indices usefult when applying filters to some index based storage.
#[derive(Debug, Clone, Default)]
pub struct Buffer {
    indices: Option<Vec<usize>>,
}

impl Buffer {
    /// If a fixed amount of indices are in the buffer returns that amount, if a non fixed amount of
    /// indices are in the buffer, such as all indices, returns none.
    pub fn count(&self) -> Option<usize> {
        self.indices.as_ref().map(Vec::len)
    }

    /// Filters the indices in place by applying a condition to the objects the buffer represents.
    pub fn filter_in_place<T>(&mut self, content: &[T], mut f: impl FnMut(&T) -> bool) -> &Self {
        let filtered = if let Some(indices) = self.indices.take() {
            indices.into_iter().filter(|i| f(&content[*i])).collect()
        } else {
            content
                .iter()
                .enumerate()
                .filter_map(|(i, v)| f(v).then_some(i))
                .collect()
        };

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
        if let Some(ref indices) = self.indices {
            either::Left(indices.clone().into_iter())
        } else {
            either::Right(0..)
        }
    }
}
