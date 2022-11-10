use std::{cell::RefCell, rc::Rc};

use crate::{
    category::Category,
    command_map::{Command, CommandErr},
};

#[derive(Debug)]
pub struct List {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl List {
    pub fn build(categories: Rc<RefCell<Vec<Category>>>) -> Box<Self> {
        Box::new(Self { categories })
    }
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
            println!("{category}");
        }

        Ok(())
    }
}
