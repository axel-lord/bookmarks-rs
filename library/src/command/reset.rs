use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    info::Info,
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Reset {
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
}

impl Command for Reset {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "reset should be used without any arguments".into(),
            ));
        }

        self.infos.write().unwrap().reset();
        self.categories.write().unwrap().reset();
        self.bookmarks.write().unwrap().reset();

        Ok(())
    }
}
