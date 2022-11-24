use crate::{
    command::load,
    command_map::{CommandMap, CommandMapBuilder},
    reset::ResetValues,
    shared,
};

use super::CommandErr;

pub fn build(
    name: String,
    reset_values: ResetValues,
    info_container: shared::Infos,
    _info_buffer: shared::Buffer,
    _selected_info: shared::Selected,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMapBuilder::new()
            .name(name)
            .push(
                "load",
                None,
                load::Load::build(info_container.clone(), reset_values),
            )
            .push("show", None, {
                let info_container = info_container.clone();
                Box::new(move |args: &[_]| {
                    if !args.is_empty() {
                        return Err(CommandErr::Execution("no info loaded".into()));
                    }

                    for (i, info) in info_container.borrow().iter().enumerate() {
                        println!("{i}. Categroies: ");
                        for category in info.categories() {
                            println!("\t{category}");
                        }
                    }

                    Ok(())
                })
            })
            .build(),
    )
}
