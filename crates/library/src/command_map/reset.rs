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

        macro_rules! reset_buffer_storage {
            ($($storage:expr),* $(,)?) => {
                $(
                    $storage.write().expect("posoned lock").reset();
                )*
            };
        }

        reset_buffer_storage!(self.infos, self.categories, self.bookmarks,);

        Ok(())
    }
}
