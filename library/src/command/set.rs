use crate::{command::Command, shared, CommandErr};
use bookmark_command::Command;
use bookmark_storage::{Property, Storeable};

#[derive(Debug, Command)]
pub struct Set<T>
where
    T: Storeable + std::fmt::Display,
{
    buffer_storage: shared::BufferStorage<T>,
}

impl<T> Command for Set<T>
where
    T: Storeable + std::fmt::Display,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() < 2 {
            return Err(CommandErr::Usage(format!(
                "set needs at least two arguments (a property and a value) {} were given",
                args.len()
            )));
        }

        let mut buffer_storage = self.buffer_storage.write().unwrap();

        let index = buffer_storage
            .selected
            .index()
            .ok_or_else(|| CommandErr::Execution("not item selected".into()))?;

        let item = buffer_storage
            .storage
            .get_mut(index)
            .ok_or_else(|| CommandErr::Execution("invalid item selected".into()))?;

        let property = args[0].as_str();

        match item.get(property) {
            Err(err) => return Err(err.into()),
            Ok(Property::List(_)) => {
                item.set(property, Property::List(Vec::from(&args[1..])))?;
            }
            Ok(Property::Single(_)) => {
                if args[1..].len() != 1 {
                    return Err(CommandErr::Execution(format!(
                        "property {} takes only a single value",
                        property
                    )));
                } else {
                    item.set(property, Property::Single(args[1].clone()))?;
                }
            }
        }

        println!("{}. {:#}", index, item);

        Ok(())
    }
}
