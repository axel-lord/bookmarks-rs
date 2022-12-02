use bookmark_storage::Storeable;

use crate::{
    command::{Command, CommandErr},
    shared,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Count<T>
where
    T: Storeable,
{
    buffer_storage: shared::BufferStorage<T>,
}

impl<T> Command for Count<T>
where
    T: Storeable,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if !args.is_empty() {
            return Err(CommandErr::Execution(
                "count should be used without any arguments".into(),
            ));
        }

        println!(
            "total: {}, in buffer: {}",
            self.buffer_storage.storage.read().unwrap().len(),
            self.buffer_storage
                .buffer
                .read()
                .unwrap()
                .count()
                .map(|b| b.to_string())
                .unwrap_or_else(|| "All".into()),
        );

        Ok(())
    }
}
