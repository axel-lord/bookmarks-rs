use std::{cell::RefCell, rc::Rc};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load<T: bookmark_storage::Storeable>
where
    T: bookmark_storage::Section,
{
    destination: Rc<RefCell<Vec<T>>>,
}
