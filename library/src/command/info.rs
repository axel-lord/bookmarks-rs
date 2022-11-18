use crate::{
    command::{Command, CommandErr},
    command_map::CommandMap,
};

#[derive(Debug, Default)]
pub struct Info {
    command_map: CommandMap<'static>,
}

impl Info {
    pub fn build(name: String) -> Box<Self> {
        Box::new(Self {
            command_map: CommandMap::new().set_name(name),
        })
    }
}

impl Command for Info {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self.command_map.call(
            &args.get(0).ok_or_else(|| {
                CommandErr::Execution("bookmark needs to be called with a subcommand".into())
            })?,
            &args[1..],
        )
    }
}
