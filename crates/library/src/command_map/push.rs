use crate::shared;
use bookmark_command::CommandErr;
use bookmark_storage::{Property, Storeable};

pub fn build<T>(buffer_storage: shared::BufferStorage<T>) -> Box<dyn bookmark_command::Command>
where
    T: Storeable + std::fmt::Display + 'static,
{
    Box::new(move |args: &[String]| {
        if args.len() < 2 {
            return Err(CommandErr::Usage(
                "push should be called with at least two arguments".into(),
            ));
        }

        let mut buffer_storage = buffer_storage.write().expect("poisoned lock");

        let (index, item) = buffer_storage
            .get_index_and_selected_mut()
            .map_err(|err| format!("{err}"))?;

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

        println!("{index}. {item:#}");

        Ok(())
    })
}
