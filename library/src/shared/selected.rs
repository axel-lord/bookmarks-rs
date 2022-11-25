use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug, Default)]
pub struct Selected(Rc<RefCell<Option<usize>>>);

impl Selected {
    pub fn get<'a, T>(&self, container: &'a [T]) -> Option<&'a T> {
        container.get(*self.0.borrow().as_ref()?)
    }

    pub fn index(&self) -> Option<usize> {
        Some(*self.0.borrow().as_ref()?)
    }

    pub fn get_mut<'a, T>(&self, container: &'a mut [T]) -> Option<&'a mut T> {
        container.get_mut(*self.0.borrow().as_ref()?)
    }

    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_none()
    }

    pub fn clear(&self) {
        self.0.borrow_mut().take();
    }

    pub fn replace(&self, value: usize) {
        self.0.borrow_mut().replace(value);
    }
}
