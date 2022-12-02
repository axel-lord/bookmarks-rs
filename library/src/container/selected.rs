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
