use crate::{
    command::{load::Load, Command, CommandErr},
    command_map::CommandMap,
    shared,
};

pub mod list;
pub mod save;
pub mod select;

#[derive(Debug, Default)]
pub struct Category {
    command_map: CommandMap<'static>,
}

impl Category {
    pub fn build(name: String, categories: shared::Categroies) -> Box<Self> {
        Box::new(Self {
            command_map: CommandMap::new()
                .set_name(name)
                .push("load", None, Load::build(categories.clone()))
                .push("list", None, list::List::build(categories.clone()))
                .push("save", None, save::Save::build(categories.clone())),
        })
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
