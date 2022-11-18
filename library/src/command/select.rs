use bookmark_storage::Storeable;

use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Select<T>
where
    T: Storeable,
{
    items: shared::Storage<T>,
    selected: shared::Selected,
}

impl<T> Command for Select<T>
where
    T: Storeable + std::fmt::Display,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "select should be called with one argument".into(),
            ));
        }

        let index = args[0].parse().map_err(|_| {
            CommandErr::Usage(format!(
                "could not parse {} as a positive integer",
                &args[0]
            ))
        })?;

        if !(..self.items.borrow().len()).contains(&index) {
            return Err(CommandErr::Execution(format!(
                "{index} is not the index of a bookmark"
            )));
        }

        self.selected.replace(index);

        println!("selected:\n{}. {:#}", index, self.items.borrow()[index]);

        Ok(())
    }
}
