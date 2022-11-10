use crate::command_map::CommandMap;

pub mod load;
pub mod save;

#[derive(Debug)]
struct Category {
    command_map: CommandMap<'static>,
}
