use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, Default)]
pub struct Selected(Arc<RwLock<Option<usize>>>);

impl Selected {
    pub fn index(&self) -> Option<usize> {
        Some(*self.0.read().unwrap().as_ref()?)
    }

    pub fn is_empty(&self) -> bool {
        self.0.read().unwrap().is_none()
    }

    pub fn clear(&self) {
        self.0.write().unwrap().take();
    }

    pub fn replace(&self, value: usize) {
        self.0.write().unwrap().replace(value);
    }
}
