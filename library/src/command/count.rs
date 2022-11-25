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
    storage: shared::Storage<T>,
    buffer: shared::Buffer,
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
            self.storage.len(),
            self.buffer
                .count()
                .map(|b| b.to_string())
                .unwrap_or_else(|| "All".into()),
        );

        Ok(())
    }
}
