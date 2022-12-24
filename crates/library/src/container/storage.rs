/// The storage of a type, used to limit the amount of available operations
/// to mutate the size storage.
#[derive(Clone, Debug, Default)]
pub struct Storage<T> {
    content: Vec<T>,
}

impl<T> Storage<T> {
    /// Push a value into the storage.
    pub fn push(&mut self, value: T) -> &mut Self {
        self.content.push(value);
        self
    }
    /// Extend the storage with an [IntoIterator] of values.
    pub fn extend(&mut self, iter: impl IntoIterator<Item = T>) -> &mut Self {
        self.content.extend(iter.into_iter());
        self
    }

    /// Remove neighboring duplicates in the storage
    pub fn dedup_by(&mut self, same_bucket: impl FnMut(&mut T, &mut T) -> bool) {
        self.content.dedup_by(same_bucket);
    }
}

impl<T> std::ops::Deref for Storage<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.content.deref()
    }
}

impl<T> std::ops::DerefMut for Storage<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.content.deref_mut()
    }
}
