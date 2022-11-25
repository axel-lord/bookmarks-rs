use bookmark_storage::{Property, Storeable};

use crate::{
    command::{Command, CommandErr},
    shared,
};

pub fn build<T>(storage: shared::Storage<T>, selected: shared::Selected) -> Box<dyn Command>
where
    T: Storeable + std::fmt::Display + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() < 2 {
            return Err(CommandErr::Usage(
                "push should be called with at least two arguments".into(),
            ));
        }

        let mut storage = storage.borrow_mut();
        let item = selected
            .get_mut(&mut storage)
            .ok_or_else(|| CommandErr::Execution("no or an invalid item selected".into()))?;

        let property = args[0].as_str();

        match item.get(property)? {
            Property::List(_) => {
                for value in &args[1..] {
                    item.push(property, value)?;
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
