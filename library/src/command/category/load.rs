use std::{cell::RefCell, fs::File, io, rc::Rc};

use crate::{
    category::{Category, CategoryErr},
    command_map::{Command, CommandErr},
    token,
};

#[derive(Debug)]
pub struct Load {
    categories: Rc<RefCell<Vec<Category>>>,
}

impl Load {
    pub fn build(categories: Rc<RefCell<Vec<Category>>>) -> Box<Self> {
        Box::new(Self { categories })
    }
}

impl Command for Load {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        if args.len() != 1 {
            return Err(CommandErr::Execution(
                "category load should be called with one argument".into(),
            ));
        }

        let Ok(file) = File::open(&args[0]) else {
            return Err(CommandErr::Execution(format!("could not open {}", &args[0])));
        };

        let Ok(content) = io::read_to_string(file) else {
            return Err(CommandErr::Execution(format!("failed to read {}", &args[0])));
        };

        let category_iter = content
            .lines()
            .enumerate()
            .skip_while(|(_, l)| !l.contains(token::CATEGORY_BEGIN))
            .skip(1)
            .take_while(|(_, l)| !l.contains(token::CATEGORY_END))
            .map(|(i, l)| Category::with_str(l.into(), Some(i)));

        let loaded = match category_iter.collect::<Result<Vec<Category>, CategoryErr>>() {
            Ok(categories) => categories,
            Err(CategoryErr::LineParseFailure(line, Some(i))) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse category on line {}: {}",
                    i, line
                )))
            }
            Err(CategoryErr::LineParseFailure(line, None)) => {
                return Err(CommandErr::Execution(format!(
                    "could not parse category from line: {}",
                    line
                )))
            }
        };

        self.categories.borrow_mut().extend_from_slice(&loaded);

        Ok(())
    }
}
