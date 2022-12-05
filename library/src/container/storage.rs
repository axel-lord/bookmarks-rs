#[derive(Clone, Debug, Default)]
pub struct Storage<T> {
    content: Vec<T>,
}

impl<T> Storage<T> {
    pub fn push(&mut self, value: T) -> &mut Self {
        self.content.push(value);
        self
    }
    pub fn extend(&mut self, iter: impl Iterator<Item = T>) -> &mut Self {
        self.content.extend(iter);
        self
    }

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