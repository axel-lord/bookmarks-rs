use std::collections::HashMap;

use bookmark_command::Command;
use bookmark_library::{
    command_map, shared::BufferStorage, Bookmark, Category, CommandFactory, IdentifierContainer,
    Info,
};
use serde::Serialize;
use tap::{Pipe, Tap};
use uuid::Uuid;

mod message_pack;
mod xml;

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
                xml::build(infos.clone(), categories.clone(), bookmarks.clone()),
            )
            .push(
                "mp",
                Some("export to message pack binary"),
                message_pack::build(infos, categories, bookmarks),
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

impl FileData {
    fn from_slices(infos: &[Info], categories: &[Category], bookmarks: &[Bookmark]) -> Self {
        let mut data = FileData::default();
        let mut top_cats = Vec::new();
        let mut cat_map = HashMap::new();

        for info in infos.iter() {
            data.tag.extend(info.tags().map(String::from));
            top_cats.extend(info.categories().map(String::from));
        }

        top_cats.sort();
        top_cats.dedup();

        for category in categories.iter().cloned() {
            cat_map.insert(String::from(category.id()), category);
        }

        data.category = top_cats
            .iter()
            .cloned()
            .map(|cat| CategoryData::from_map(&cat_map, cat))
            .collect::<Vec<_>>();

        data.bookmark = bookmarks
            .iter()
            .map(|b| BookmarkData {
                url: b.url().into(),
                info: b.description().into(),
                tag: b.tags().map(String::from).collect(),
                uuid: Uuid::new_v4(),
            })
            .collect::<Vec<_>>()
            .tap_mut(|vec| vec.sort_by_key(|b| b.url.clone()))
            .tap_mut(|vec| vec.dedup_by_key(|b| b.url.clone()));
        data
    }
}

impl<'a> From<IdentifierContainer<'a>> for IdentifierData {
    fn from(value: IdentifierContainer<'a>) -> Self {
        let transform_vec = |vec: Vec<&str>| vec.into_iter().map(String::from).collect();
        Self {
            require: value.require.pipe(transform_vec),
            whole: value.whole.pipe(transform_vec),
            include: value.include.pipe(transform_vec),
        }
    }
}

impl CategoryData {
    fn from_map(map: &HashMap<String, Category>, id: String) -> Self {
        Self {
            name: map[&id].name().into(),
            info: map[&id].description().into(),
            identifier: map[&id]
                .identifier_container()
                .expect(
                    "there should be not reason for this IdentifierContainer to not be creatable",
                )
                .into(),
            subcategory: map[&id]
                .subcategories()
                .map(|sub_id| Self::from_map(map, sub_id.into()))
                .collect(),
        }
    }
}
#[derive(Default, Debug, Serialize)]
struct FileData {
    tag: Vec<String>,
    category: Vec<CategoryData>,
    bookmark: Vec<BookmarkData>,
}

#[derive(Default, Debug, Serialize)]
struct CategoryData {
    name: String,
    info: String,
    identifier: IdentifierData,
    subcategory: Vec<CategoryData>,
}

#[derive(Default, Debug, Serialize)]
struct IdentifierData {
    require: Vec<String>,
    whole: Vec<String>,
    include: Vec<String>,
}

#[derive(Default, Debug, Serialize)]
struct BookmarkData {
    url: String,
    info: String,
    uuid: Uuid,
    tag: Vec<String>,
}
