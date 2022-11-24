pub mod apply;

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{count, list, load, print, push, save, select, set},
    command_map::{CommandMap, CommandMapBuilder},
    reset::ResetValues,
    shared,
};

pub fn build(
    name: String,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
    reset_values: ResetValues,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMapBuilder::new()
            .name(name)
            .push(
                "load",
                None,
                load::Load::build(categories.storage.clone(), reset_values.clone()),
            )
            .push(
                "list",
                Some("list categories"),
                list::List::build(categories.storage.clone(), categories.buffer.clone()),
            )
            .push(
                "count",
                Some("count amount of categories"),
                count::Count::build(categories.storage.clone(), categories.buffer.clone()),
            )
            .push(
                "set",
                None,
                set::Set::build(categories.storage.clone(), categories.selected.clone()),
            )
            .push(
                "save",
                None,
                save::Save::build(categories.storage.clone(), categories.buffer.clone()),
            )
            .push(
                "print",
                Some("print selected category"),
                print::build(categories.storage.clone(), categories.selected.clone()),
            )
            .push(
                "push",
                Some("push a value onto a list field"),
                push::build(categories.storage.clone(), categories.selected.clone()),
            )
            .push(
                "select",
                None,
                select::Select::build(categories.storage.clone(), categories.selected.clone()),
            )
            .push(
                "apply",
                Some("filter bookmarks in buffer by selected category"),
                apply::build(bookmarks.clone(), categories.clone()),
            )
            .build(),
    )
}
