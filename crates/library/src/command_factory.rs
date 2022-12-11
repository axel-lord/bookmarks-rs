use crate::{bookmark::Bookmark, category::Category, info::Info, shared};
use bookmark_command::Command;

/// Type for storing command creators for to facilitate
/// building the starting command map.
pub trait CommandFactory {
    /// Get the name of the command that is to be created most probaly will be used for calling it
    /// so make sure to avoid collisions.
    fn name(&self) -> &'static str;

    /// Create the command, easily the most important function in the type.
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
    ) -> Box<dyn Command>;

    /// What help message will go with the command, whilst optional should be implemented.
    fn help(&self) -> Option<&'static str> {
        None
    }
}

impl<F> CommandFactory for (&'static str, F)
where
    F: FnMut(
        shared::BufferStorage<Bookmark>,
        shared::BufferStorage<Category>,
        shared::BufferStorage<Info>,
    ) -> Box<dyn Command>,
{
    fn name(&self) -> &'static str {
        self.0
    }
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
    ) -> Box<dyn Command> {
        (self.1)(bookmarks, categories, infos)
    }
}

impl<F> CommandFactory for (&'static str, &'static str, F)
where
    F: FnMut(
        shared::BufferStorage<Bookmark>,
        shared::BufferStorage<Category>,
        shared::BufferStorage<Info>,
    ) -> Box<dyn Command>,
{
    fn name(&self) -> &'static str {
        self.0
    }
    fn help(&self) -> Option<&'static str> {
        Some(self.1)
    }
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
    ) -> Box<dyn Command> {
        (self.2)(bookmarks, categories, infos)
    }
}
