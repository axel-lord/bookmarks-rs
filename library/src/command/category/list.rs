use std::{cell::RefCell, rc::Rc};

use crate::{
    category::Category,
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct List {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Command for List {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 0 {
            return Err(CommandErr::Execution(
                "category list should be called without any arguments".into(),
            ));
        }

        println!("listing all categories");

        for category in self.categories.borrow().iter() {
            println!("{}", category.name());
        }

        Ok(())
    }
}
