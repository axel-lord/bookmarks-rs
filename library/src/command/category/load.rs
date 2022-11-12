use std::{cell::RefCell, rc::Rc};

use crate::{
    category::Category,
    command_map::{Command, CommandErr},
    load, token,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Load {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Command for Load {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "category load should be called with one argument".into(),
            ));
        }

        let loaded = load::load(
            &args[0],
            token::CATEGORY_BEGIN,
            token::CATEGORY_END,
            Category::with_str,
        )
        .map_err(|err| CommandErr::Execution(format!("in file {}: {}", &args[0], err)))?;

        if loaded.is_empty() {
            return Err(CommandErr::Execution(format!(
                "no categories parsed from {}",
                &args[0]
            )));
        }

        self.categories.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
