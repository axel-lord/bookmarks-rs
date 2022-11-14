use std::{cell::RefCell, fs::File, io::BufWriter, rc::Rc};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct SaveAll {
    categories: Rc<RefCell<Vec<Category>>>,
    bookmarks: Rc<RefCell<Vec<Bookmark>>>,
}

impl Command for SaveAll {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        let mut writer = BufWriter::new(File::create(&args[0])?);

        bookmark_storage::save(&mut writer, self.categories.borrow().iter())?;

        bookmark_storage::save(&mut writer, self.bookmarks.borrow().iter())?;

        Ok(())
    }
}
