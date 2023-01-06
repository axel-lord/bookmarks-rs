use crate::shared;
use bookmark_command::Command;
use bookmark_storage::Storeable;

#[derive(Debug, Command)]
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
    fn call(&mut self, args: &[String]) -> Result<(), bookmark_command::CommandErr> {
        if !args.is_empty() {
            return Err(bookmark_command::CommandErr::Execution(
                "count should be used without any arguments".into(),
            ));
        }

        let buffer_storage = self
            .buffer_storage
            .read()
            .expect("failed to aquire read lock");

        println!(
            "total: {}, in buffer: {}",
            buffer_storage.storage.len(),
            buffer_storage
                .buffer
                .count()
                .map_or_else(|| "All".into(), |b| b.to_string()),
        );

        Ok(())
    }
}
