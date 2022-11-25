use crate::{
    command::{Command, CommandErr},
    shared,
};

use bookmark_storage::Storeable;

pub fn build<T>(storage: shared::Storage<T>, selected: shared::Selected) -> Box<dyn Command>
where
    T: 'static + Storeable + std::fmt::Display,
{
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "print should be called without any arguments".into(),
            ));
        }
        if selected.is_empty() {
            return Err(CommandErr::Execution("noting selected".into()));
        }

        let bookmarks = storage.borrow();

        let Some(selected_item) = selected.get(&bookmarks) else {
            return Err(CommandErr::Execution("selected item does not exist".into()));
        };

        println!("{}. {:#}", selected.index().unwrap(), selected_item);

        Ok(())
    })
}
