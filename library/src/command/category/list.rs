use std::{cell::RefCell, rc::Rc};

use crate::{
    category::Category,
    command::list,
    command::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Command for List {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        let categories = self.categories.borrow();

        let count = args
            .get(0)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(categories.len()))
            .map_err(|_| {
                CommandErr::Execution(format!(
                    "could not parse {} as a positive integer",
                    &args[0]
                ))
            })?;

        let from = args
            .get(1)
            .map(|arg| arg.parse())
            .unwrap_or(Ok(0isize))
            .map_err(|_| {
                CommandErr::Execution(format!("could not parse {} as an integer", &args[1]))
            })
            .map(|from| list::wrap_if_negative(from, categories.len()))??;

        println!("listing {count} categories starting at index {from}");

        for (index, category) in categories.iter().enumerate().skip(from).take(count) {
            print!("{}. {:#}", index, category);
        }

        Ok(())
    }
}
