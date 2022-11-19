use crate::{
    command::{count, list, load, save, select, set},
    command_map::CommandMap,
    reset::ResetValues,
    shared,
};

pub fn build(
    name: String,
    categories: shared::Categroies,
    category_buffer: shared::Buffer,
    selected_category: shared::Selected,
    reset_values: ResetValues,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMap::new()
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
                "count",
                None,
                count::Count::build(categories.clone(), category_buffer.clone()),
            )
            .push(
                "set",
                None,
                set::Set::build(categories.clone(), selected_category.clone()),
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
    )
}
