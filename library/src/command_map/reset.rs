use crate::{bookmark::Bookmark, category::Category, info::Info, shared};
use bookmark_command::Command;

#[derive(Debug, Command)]
pub struct Reset {
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for Reset {
    fn call(&mut self, args: &[String]) -> Result<(), bookmark_command::CommandErr> {
        if !args.is_empty() {
            return Err(bookmark_command::CommandErr::Execution(
                "reset should be used without any arguments".into(),
            ));
        }

        self.infos.write().unwrap().reset();
        self.categories.write().unwrap().reset();
        self.bookmarks.write().unwrap().reset();

        Ok(())
    }
}
