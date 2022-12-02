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
    pub fn count(&self) -> Option<usize> {
        self.indices.as_ref().map(Vec::len)
    }

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

    pub fn reset(&mut self) -> &mut Self {
        self.indices.take();
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> {
        self.indices
            .as_ref()
            .map(|v| EitherIter::A(v.clone().into_iter()))
            .unwrap_or_else(|| EitherIter::B(0..))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Selected {
    index: Option<usize>,
}

impl Selected {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_none()
    }

    pub fn clear(&mut self) -> &mut Self {
        self.index.take();
        self
    }

    pub fn replace(&mut self, value: usize) -> &mut Self {
        self.index.replace(value);
        self
    }
}

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
