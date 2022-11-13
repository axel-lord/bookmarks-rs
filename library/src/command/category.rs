use std::{cell::RefCell, rc::Rc};

use crate::{
    category,
    command::load::Load,
    command_map::{Command, CommandErr, CommandMap},
};

pub mod list;
pub mod save;

#[derive(Debug, Default)]
pub struct Category {
    command_map: CommandMap<'static>,
}

impl Category {
    pub fn build(categories: Rc<RefCell<Vec<category::Category>>>) -> Box<Self> {
        let mut subcommand: Box<Self> = Default::default();
        let command_map = &mut subcommand.command_map;

        command_map.push("load", None, Load::build(categories.clone()));

        command_map.push("list", None, list::List::build(categories.clone()));

        command_map.push("save", None, save::Save::build(categories.clone()));

        subcommand
    }
}

impl Command for Category {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self.command_map.call(
            &args.get(0).ok_or_else(|| {
                CommandErr::Execution("category needs to be called with a subcommand".into())
            })?,
            &args[1..],
        )
    }
}
