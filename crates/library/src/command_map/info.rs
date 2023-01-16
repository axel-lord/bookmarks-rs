use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use bookmark_command::args_are_empty;

use crate::{
    category::Category,
    command_map::{load, Builder as CommandMapBuilder, CommandMap},
    info::Info,
    shared,
};

#[derive(Default)]
struct CatNode {
    name: String,
    parents: Vec<Weak<RefCell<CatNode>>>,
    children: Vec<Weak<RefCell<CatNode>>>,
}

impl CatNode {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
}

#[derive(Default)]
struct CatMap {
    map: HashMap<String, Rc<RefCell<CatNode>>>,
}

impl CatMap {
    fn get_or_create(&mut self, key: &str) -> Rc<RefCell<CatNode>> {
        self.map
            .entry(key.into())
            .or_insert_with(|| Rc::new(RefCell::new(CatNode::new(key.into()))))
            .clone()
    }

    fn new() -> Self {
        CatMap::default()
    }
}

pub fn build(
    name: String,
    infos: shared::BufferStorage<Info>,
    categories: shared::BufferStorage<Category>,
) -> Box<CommandMap<'static>> {
    Box::new(
        CommandMapBuilder::new()
            .name(name)
            .push("load", None, load::Load::build(infos.clone()))
            .push("categories", Some("show category hierarchy"), {
                let categories = categories;
                let infos = infos.clone();
                Box::new(move |args: &[_]| {
                    args_are_empty(args)?;

                    let mut map = CatMap::new();
                    for cat in categories.read().storage.iter() {
                        let cat_entry = map.get_or_create(cat.id());

                        for child in cat.subcategories() {
                            let child_entry = map.get_or_create(child);

                            child_entry
                                .borrow_mut()
                                .parents
                                .push(Rc::downgrade(&cat_entry));
                            cat_entry
                                .borrow_mut()
                                .children
                                .push(Rc::downgrade(&child_entry));
                        }
                    }

                    let mut cat_stack = Vec::new();
                    for info in infos.read().storage.iter() {
                        for cat in info.categories().collect::<Vec<_>>().into_iter().rev() {
                            cat_stack.push((0usize, map.get_or_create(cat)));
                        }
                    }

                    while let Some((level, current)) = cat_stack.pop() {
                        let current = current.borrow();

                        println!("{}{}", "   ".repeat(level), current.name);

                        // safeguard for recursion
                        if level > 16 {
                            break;
                        }

                        for child in current.children.iter().rev() {
                            cat_stack.push((
                                level + 1,
                                child.upgrade().expect("failed to upgrade a weak ref"),
                            ));
                        }
                    }

                    Ok(())
                })
            })
            .push("show", None, {
                let infos = infos;
                Box::new(move |args: &[_]| {
                    if !args.is_empty() {
                        return Err(bookmark_command::CommandErr::Execution(
                            "no info loaded".into(),
                        ));
                    }

                    for (i, info) in infos.read().storage.iter().enumerate() {
                        println!("{i}. Categroies: ");
                        for category in info.categories() {
                            println!("\t{category}");
                        }
                    }

                    Ok(())
                })
            })
            .build(),
    )
}
