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

        let buffer_storage = self.buffer_storage.read().unwrap();

        println!(
            "total: {}, in buffer: {}",
            buffer_storage.storage.len(),
            buffer_storage
                .buffer
                .count()
                .map(|b| b.to_string())
                .unwrap_or_else(|| "All".into()),
        );

        Ok(())
    }
}
