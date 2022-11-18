use crate::{
    command::{list, load, save, select, Command, CommandErr},
    command_map::CommandMap,
    reset::ResetValues,
    shared,
};

#[derive(Debug, Default)]
pub struct Category {
    command_map: CommandMap<'static>,
}

impl Category {
    pub fn build(
        name: String,
        categories: shared::Categroies,
        category_buffer: shared::Buffer,
        selected_category: shared::Selected,
        reset_values: ResetValues,
    ) -> Box<Self> {
        Box::new(Self {
            command_map: CommandMap::new()
                .set_name(name)
                .push(
                    "load",
                    None,
                    load::Load::build(categories.clone(), reset_values.clone()),
                )
                .push(
                    "list",
                    None,
                    list::List::build(categories.clone(), category_buffer.clone()),
                )
                .push(
                    "save",
                    None,
                    save::Save::build(categories.clone(), category_buffer.clone()),
                )
                .push(
                    "select",
                    None,
                    select::Select::build(categories.clone(), selected_category.clone()),
                ),
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
