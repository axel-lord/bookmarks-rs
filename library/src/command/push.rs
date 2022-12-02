use bookmark_storage::{Property, Storeable};

use crate::{
    command::{Command, CommandErr},
    shared,
};

pub fn build<T>(buffer_storage: shared::BufferStorage<T>) -> Box<dyn Command>
where
    T: Storeable + std::fmt::Display + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() < 2 {
            return Err(CommandErr::Usage(
                "push should be called with at least two arguments".into(),
            ));
        }

        let selected = buffer_storage.selected.read().unwrap();
        let mut storage = buffer_storage.storage.write().unwrap();
        let item = storage
            .get_mut(
                selected
                    .index()
                    .ok_or_else(|| CommandErr::Execution("no item selected".into()))?,
            )
            .ok_or_else(|| CommandErr::Execution("invalid item selected".into()))?;

        let property_name = args[0].as_str();
        let property = item.get(property_name);

        match property? {
            Property::List(_) => {
                for value in &args[1..] {
                    item.push(property_name, value)?;
                }
            }
            Property::Single(_) => {
                return Err(CommandErr::Execution(
                    "push can only be used on list properties".into(),
                ));
            }
        }

        println!("{}. {:#}", selected.index().unwrap(), item);

        Ok(())
    })
}
