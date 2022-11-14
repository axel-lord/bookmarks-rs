use std::{cell::RefCell, fs::File, io::BufWriter, rc::Rc};

use crate::{
    category::Category,
    command_map::{Command, CommandErr},
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Command for Save {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "save should be called with one argument".into(),
            ));
        }

        bookmark_storage::save(
            &mut BufWriter::new(File::create(&args[0])?),
            self.categories.borrow().iter(),
        )?;

        Ok(())
    }
}
