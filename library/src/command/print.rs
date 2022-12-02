use crate::{
    command::{Command, CommandErr},
    shared,
};

use bookmark_storage::Storeable;

pub fn build<T>(buffer_storage: shared::BufferStorage<T>) -> Box<dyn Command>
where
    T: 'static + Storeable + std::fmt::Display,
{
    Box::new(move |args: &[_]| {
        if !args.is_empty() {
            return Err(CommandErr::Usage(
                "print should be called without any arguments".into(),
            ));
        }

        let selected = buffer_storage.selected.read().unwrap();

        if selected.is_empty() {
            return Err(CommandErr::Execution("noting selected".into()));
        }

        println!(
            "{}. {:#}",
            selected.index().unwrap(),
            buffer_storage
                .storage
                .read()
                .unwrap()
                .get(
                    selected.index().ok_or_else(|| CommandErr::Execution(
                        "selected item does not exist".into()
                    ))?
                )
                .unwrap()
        );

        Ok(())
    })
}
