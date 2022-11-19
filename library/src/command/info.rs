use crate::{command::load, command_map::CommandMap, reset::ResetValues, shared::Infos};

use super::CommandErr;

pub fn build(name: String, reset_values: ResetValues) -> Box<CommandMap<'static>> {
    let info_container = Infos::default();
    Box::new(
        CommandMap::new()
            .set_name(name)
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
            }),
    )
}
