use crate::{command::Command, shared, CommandErr};
use bookmark_storage::{Property, Storeable};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Set<T>
where
    T: Storeable + std::fmt::Display,
{
    storage: shared::Storage<T>,
    selected: shared::Selected,
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

        let item = self
            .selected
            .get_mut(&mut self.storage)
            .ok_or_else(|| CommandErr::Execution("no or an invalid item selected".into()))?;

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

        println!("{}. {:#}", self.selected.index().unwrap(), item);

        Ok(())
    }
}
