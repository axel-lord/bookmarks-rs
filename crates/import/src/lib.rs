//! Commands for importing bookmarks from foreign formats.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

mod html;
mod json;
mod onetab;

use bookmark_library::{command_map::CommandMapBuilder, shared::BufferStorage};

/// Type used to build import command.
#[derive(Debug, Clone, Copy)]
pub struct Import;

impl bookmark_library::CommandFactory for Import {
    fn name(&self) -> &'static str {
        "import"
    }
    fn build(
        &mut self,
        bookmarks: BufferStorage<bookmark_library::Bookmark>,
        _categories: BufferStorage<bookmark_library::Category>,
        _infos: BufferStorage<bookmark_library::Info>,
    ) -> Box<dyn bookmark_command::Command> {
        Box::new(
            CommandMapBuilder::new()
                .name("import".into())
                .push(
                    "onetab",
                    Some("import a onetab export"),
                    onetab::build(bookmarks.clone()),
                )
                .push(
                    "html",
                    Some("import a firefox html export"),
                    html::build(bookmarks.clone()),
                )
                .push(
                    "json",
                    Some("parse firefox bookmark backup"),
                    json::build(bookmarks),
                )
                .build(),
        )
    }
}
