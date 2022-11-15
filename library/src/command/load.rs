use std::{cell::RefCell, fs::File, ops::Range, rc::Rc};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{Command, CommandErr},
    reset, shared,
};

use bookmark_storage::Listed;

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load<T>
where
    T: Listed + Clone,
{
    destination: shared::Storage<T>,
}

impl<T> Command for Load<T>
where
    T: Listed + Clone,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no lines parsed from {}",
                &args[0]
            )));
        }

        self.destination.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct LoadAll {
    categories: shared::Categroies,
    bookmarks: shared::Bookmarks,
    buffer: shared::Buffer,
    selected_bookmark: shared::Selected,
}

impl Command for LoadAll {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no category lines parsed from {}",
                &args[0]
            )));
        }

        self.categories.borrow_mut().extend(loaded.into_iter());

        let loaded = bookmark_storage::load(&mut File::open(&args[0])?)?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no bookmark lines parsed from {}",
                &args[0]
            )));
        }

        self.bookmarks.borrow_mut().extend(loaded.into_iter());

        reset::reset(
            &mut self.buffer.borrow_mut(),
            &self.bookmarks.borrow(),
            &mut self.selected_bookmark.borrow_mut(),
        );

        Ok(())
    }
}
