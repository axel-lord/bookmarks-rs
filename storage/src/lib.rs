pub trait Storeable: Sized {
    fn is_edited(&self) -> bool;
}
