use crate::{
    command::{count, list, load, print, push, save, select, set},
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
                Some("list categories"),
                list::List::build(categories.clone(), category_buffer.clone()),
            )
            .push(
                "count",
                Some("count amount of categories"),
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
                "print",
                Some("print selected category"),
                print::build(categories.clone(), selected_category.clone()),
            )
            .push(
                "push",
                Some("push a value onto a list field"),
                push::build(categories.clone(), selected_category.clone()),
            )
            .push(
                "select",
                None,
                select::Select::build(categories.clone(), selected_category.clone()),
            ),
    )
}
