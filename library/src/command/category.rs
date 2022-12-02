pub mod apply;

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{count, list, load, print, push, save, select, set},
    command_map::{CommandMap, CommandMapBuilder},
    shared,
};

pub fn build(
    name: String,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMapBuilder::new()
            .name(name)
            .push("load", None, load::Load::build(categories.clone()))
            .push(
                "list",
                Some("list categories"),
                list::List::build(categories.clone()),
            )
            .push(
                "count",
                Some("count amount of categories"),
                count::Count::build(categories.clone()),
            )
            .push("set", None, set::Set::build(categories.clone()))
            .push("save", None, save::Save::build(categories.clone()))
            .push(
                "print",
                Some("print selected category"),
                print::build(categories.clone()),
            )
            .push(
                "push",
                Some("push a value onto a list field"),
                push::build(categories.clone()),
            )
            .push("select", None, select::Select::build(categories.clone()))
            .push(
                "apply",
                Some("filter bookmarks in buffer by selected category"),
                apply::build(bookmarks /*.clone()*/, categories /*.clone()*/),
            )
            .build(),
    )
}
