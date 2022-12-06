/// Type representi a selected item in an indice based container.
#[derive(Debug, Clone, Copy, Default)]
pub struct Selected {
    index: Option<usize>,
}

impl Selected {
    /// Get the index of the selected item, if something is selected.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Returns true if nothing is selected.
    pub fn is_empty(&self) -> bool {
        self.index.is_none()
    }

    /// Deselect whatever is selected.
    pub fn clear(&mut self) -> &mut Self {
        self.index.take();
        self
    }

    /// Replace whatever index (if any) is currently stored with a new one.
    pub fn replace(&mut self, value: usize) -> &mut Self {
        self.index.replace(value);
        self
    }
}
