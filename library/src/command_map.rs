use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{self, Command, CommandErr},
};

struct CommandEntry {
    command: RefCell<Box<dyn Command>>,
    help: Option<String>,
}

impl CommandEntry {
    fn new(command: Box<dyn Command>, help: Option<&str>) -> Self {
        CommandEntry {
            command: RefCell::new(command),
            help: help.map(String::from),
        }
    }
}

impl Debug for CommandEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(help) = self.help.as_ref() {
            write!(f, "<{}>", help)
        } else {
            write!(f, "<>")
        }
    }
}

#[derive(Default, Debug)]
pub struct CommandMap<'a>(HashMap<&'a str, CommandEntry>);

impl<'a> CommandMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, name: &'a str, help: Option<&str>, command: Box<dyn Command>) {
        self.0.insert(name, CommandEntry::new(command, help));
    }

    pub fn call(&self, name: &str, args: &[String]) -> Result<(), CommandErr> {
        if let Some(command) = self.0.get(name) {
            command.command.borrow_mut().call(args)
        } else {
            Err(CommandErr::Lookup)
        }
    }
}

impl CommandMap<'static> {
    pub fn build(
        bookmarks: Rc<RefCell<Vec<Bookmark>>>,
        categories: Rc<RefCell<Vec<Category>>>,
    ) -> Self {
        let mut command_map = Self::new();
        let buffer: Rc<RefCell<_>> = Default::default();
        crate::reset::reset(&mut buffer.borrow_mut(), &bookmarks.borrow());

        use command::*;

        command_map.push(
            "reset",
            None,
            reset::Reset::build(bookmarks.clone(), buffer.clone()),
        );

        command_map.push(
            "category",
            None,
            category::Category::build(categories.clone()),
        );

        command_map.push(
            "bookmark",
            None,
            bookmark::Bookmark::build(bookmarks.clone(), buffer.clone()),
        );

        command_map.push(
            "load",
            None,
            load::LoadAll::build(categories.clone(), bookmarks.clone(), buffer.clone()),
        );

        command_map.push(
            "save",
            None,
            save::SaveAll::build(categories.clone(), bookmarks.clone()),
        );

        command_map.push("debug", None, Box::new(command_debug));

        command_map
    }
}
