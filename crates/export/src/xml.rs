use bookmark_command::{Command, CommandErr};
use bookmark_library::{shared, Bookmark, Category, IdentifierContainer, Info};
use serde::Serialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufWriter},
};
use tap::{Pipe, Tap};
use uuid::Uuid;

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

pub fn build(
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
    bookmarks: shared::BufferStorage<Bookmark>,
) -> Box<dyn Command> {
    Box::new(move |args: &[String]| {
        if args.len() != 1 {
            return Err(CommandErr::Usage(
                "export xml should be given a file path".into(),
            ));
        }

        let infos = infos.read();
        let categories = categories.read();
        let bookmarks = bookmarks.read();

        let mut data = FileData::default();
        let mut top_cats = Vec::new();
        let mut cat_map = HashMap::new();

        for info in infos.storage.iter() {
            data.tag.extend(info.tags().map(String::from));
            top_cats.extend(info.categories().map(String::from));
        }

        top_cats.sort();
        top_cats.dedup();

        for category in categories.storage.iter().cloned() {
            cat_map.insert(String::from(category.id()), category);
        }

        data.category = top_cats
            .iter()
            .cloned()
            .map(|cat| CategoryData::from_map(&cat_map, cat))
            .collect::<Vec<_>>();

        data.bookmark = bookmarks
            .storage
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

        File::create(args[0].clone())?
            .pipe(BufWriter::new)
            .write_all(
                data.pipe_ref(quick_xml::se::to_string)
                    .map_err(|err| CommandErr::Execution(err.to_string()))?
                    .as_bytes(),
            )
            .map_err(|err| CommandErr::Execution(err.to_string()))?;

        Ok(())
    })
}
