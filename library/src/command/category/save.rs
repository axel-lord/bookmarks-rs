use std::{
    cell::RefCell,
    fs::File,
    io::{prelude::*, BufWriter},
    rc::Rc,
};

use crate::{
    category::Category,
    command_map::{Command, CommandErr},
    token,
};

#[derive(Debug, bookmark_derive::BuildCommand)]
pub struct Save {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Command for Save {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "category save should be called with one argument".into(),
            ));
        }

        let file = File::create(&args[0]).map_err(|err| {
            CommandErr::Execution(format!("could not open {} for reading: {}", &args[0], err))
        })?;

        let mut writer = BufWriter::new(file);

        let write_err =
            |err| CommandErr::Execution(format!("write to {} failed: {}", &args[0], err));

        write!(writer, "{}", token::CATEGORY_BEGIN).map_err(write_err)?;

        for category in self.categories.borrow().iter() {
            writeln!(writer, "{}", category.to_line()).map_err(write_err)?;
        }

        write!(writer, "{}", token::CATEGORY_END).map_err(write_err)?;

        Ok(())
    }
}
