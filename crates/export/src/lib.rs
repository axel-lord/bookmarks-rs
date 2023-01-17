use bookmark_command::Command;
use bookmark_library::{command_map, shared::BufferStorage, CommandFactory};
use tap::Pipe;

pub mod xml;

#[derive(Debug, Clone, Copy)]
pub struct Export;

impl CommandFactory for Export {
    fn name(&self) -> &'static str {
        "export"
    }

    fn build(
        &mut self,
        bookmarks: BufferStorage<bookmark_library::Bookmark>,
        categories: BufferStorage<bookmark_library::Category>,
        infos: BufferStorage<bookmark_library::Info>,
    ) -> Box<dyn Command> {
        command_map::Builder::new()
            .name("import".into())
            .push(
                "xml",
                Some("export to an xml file"),
                xml::build(infos, categories, bookmarks),
            )
            .build()
            .pipe(Box::new)
    }
}

impl Export {
    pub fn as_box() -> Box<dyn CommandFactory> {
        Box::new(Self)
    }
}
