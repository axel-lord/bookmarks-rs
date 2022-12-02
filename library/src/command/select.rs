use bookmark_storage::Storeable;

use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Select<T> {
    buffer_storage: shared::BufferStorage<T>,
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

        let mut buffer_storage = self.buffer_storage.write().unwrap();

        let selected_item = buffer_storage
            .storage
            .get(index)
            .ok_or_else(|| CommandErr::Execution(format!("{index} is not a valid index")))?;

        println!("selected:\n{}. {:#}", index, selected_item);
        buffer_storage.selected.replace(index);

        Ok(())
    }
}
