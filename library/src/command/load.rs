use std::{cell::RefCell, rc::Rc};

use crate::command_map::{Command, CommandErr};

use bookmark_storage::Listed;

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load<T>
where
    T: Listed + Clone,
{
    destination: Rc<RefCell<Vec<T>>>,
}

impl<T> Command for Load<T>
where
    T: Listed + Clone,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "load should be called with one argument".into(),
            ));
        }

        let loaded = bookmark_storage::load(&args[0])?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no lines parsed from {}",
                &args[0]
            )));
        }

        self.destination.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
