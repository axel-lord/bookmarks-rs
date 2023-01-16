use crate::shared;
use bookmark_command::CommandErr;
use bookmark_storage::Storeable;

pub fn build<T>(buffer_storage: shared::BufferStorage<T>) -> Box<dyn bookmark_command::Command>
where
    T: 'static + Storeable + std::fmt::Display,
{
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "print should be called without any arguments".into(),
            ));
        }

        let buffer_storage = buffer_storage.read();

        let index = buffer_storage
            .selected
            .index()
            .ok_or_else(|| CommandErr::Execution("nothing selected".into()))?;

        println!(
            "{}. {:#}",
            index,
            buffer_storage
                .storage
                .get(index)
                .ok_or_else(|| CommandErr::Execution("selected item does not exist".into()))?
        );

        Ok(())
    })
}
