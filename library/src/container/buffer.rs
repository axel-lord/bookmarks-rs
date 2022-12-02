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
