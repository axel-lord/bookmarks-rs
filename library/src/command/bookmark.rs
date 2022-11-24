pub mod filter;
pub mod new;
pub mod regex;
pub mod sort;
pub mod unique;

use crate::{
    command::{count, list, load, print, push, save, select, set},
    command_map::{CommandMap, CommandMapBuilder},
    reset::ResetValues,
    shared,
};

pub fn build(
    name: String,
    bookmarks: shared::Bookmarks,
    bookmark_buffer: shared::Buffer,
    selected_bookmark: shared::Selected,
    reset_values: ResetValues,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMapBuilder::new()
            .name(name)
            .push(
                "list",
                None,
                list::List::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "filter",
                None,
                filter::Filter::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "filter-inv",
                None,
                filter::FilterInv::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "regex",
                None,
                regex::Regex::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "regex-inv",
                None,
                regex::RegexInv::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "count",
                None,
                count::Count::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "load",
                None,
                load::Load::build(bookmarks.clone(), reset_values.clone()),
            )
            .push(
                "save",
                None,
                save::Save::build(bookmarks.clone(), bookmark_buffer.clone()),
            )
            .push(
                "select",
                Some("select a bookmark\nusage: select INDEX"),
                select::Select::build(bookmarks.clone(), selected_bookmark.clone()),
            )
            .push(
                "print",
                Some("print selected bookmark\nusage: print"),
                print::build(bookmarks.clone(), selected_bookmark.clone()),
            )
            .push(
                "push",
                Some("print selected bookmark\nusage: push FIELD [VALUE, ...]"),
                push::build(bookmarks.clone(), selected_bookmark.clone()),
            )
            .push(
                "new",
                Some("add a new empty bookmark"),
                new::New::build(
                    bookmarks.clone(),
                    selected_bookmark.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "set",
                Some("set a value on a bookmark\nusage: set VALUE [VALUES, [...]]"),
                set::Set::build(bookmarks.clone(), selected_bookmark.clone()),
            )
            .push(
                "sort",
                Some("sort bookmarks by url"),
                sort::build(bookmarks.clone(), reset_values.clone()),
            )
            .push(
                "unique",
                Some("sort bookmarks and remove duplicates"),
                unique::build(bookmarks.clone(), reset_values.clone()),
            )
            .build(),
    )
}
