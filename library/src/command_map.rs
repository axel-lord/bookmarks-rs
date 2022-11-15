use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use crate::{
    command::{self, Command, CommandErr},
    shared,
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
pub struct CommandMap<'a>(HashMap<&'a str, CommandEntry>, String);

impl<'a> CommandMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, name: &'a str, help: Option<&str>, command: Box<dyn Command>) {
        self.0.insert(name, CommandEntry::new(command, help));
    }

    pub fn set_name(&mut self, name: String) {
        self.1 = name;
    }

    pub fn name(&self) -> &str {
        &self.1
    }

    pub fn call(&self, name: &str, args: &[String]) -> Result<(), CommandErr> {
        match name {
            "help" => {
                if args.len() != 1 {
                    Err(CommandErr::Usage(
                        "help called with incorrect number of arguments".into(),
                    ))
                } else {
                    let command = &args[0];
                    if let Some(help) = self.help(command) {
                        println!("{help}");
                        Ok(())
                    } else {
                        Err(CommandErr::Execution(format!(
                            "found no help for {command}"
                        )))
                    }
                }
            }
            _ => {
                if let Some(command) = self.0.get(name) {
                    command.command.borrow_mut().call(args)
                } else {
                    Err(CommandErr::Lookup)
                }
            }
        }
    }

    pub fn help(&self, name: &str) -> Option<String> {
        if name == "help" {
            Some(if self.1.len() == 0 {
                "show help for a command\nusage: help COMMAND".into()
            } else {
                format!("show help for a command\nusage: {} help COMMAND", self.1)
            })
        } else {
            self.0.get(name)?.help.clone()
        }
    }
}

impl CommandMap<'static> {
    pub fn build(bookmarks: shared::Bookmarks, categories: shared::Categroies) -> Self {
        let mut command_map = Self::new();
        let buffer = shared::Buffer::default();
        let selected_bookmark = shared::Selected::default();

        crate::reset::reset(
            &mut buffer.borrow_mut(),
            &bookmarks.borrow(),
            &mut selected_bookmark.borrow_mut(),
        );

        use command::*;

        command_map.push(
            "reset",
            None,
            reset::Reset::build(bookmarks.clone(), buffer.clone(), selected_bookmark.clone()),
        );

        command_map.push(
            "category",
            None,
            category::Category::build("category".into(), categories.clone()),
        );

        command_map.push(
            "bookmark",
            None,
            bookmark::Bookmark::build(
                "bookmark".into(),
                bookmarks.clone(),
                buffer.clone(),
                selected_bookmark.clone(),
            ),
        );

        command_map.push(
            "list",
            Some("shorthand for bookmark list\nusage: list [COUNT [FROM]]"),
            command::bookmark::list::List::build(bookmarks.clone(), buffer.clone()),
        );

        command_map.push(
            "load",
            None,
            load::LoadAll::build(
                categories.clone(),
                bookmarks.clone(),
                buffer.clone(),
                selected_bookmark.clone(),
            ),
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
