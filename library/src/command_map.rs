use std::{cell::RefCell, collections::HashMap, fmt::Debug};

use crate::{
    command::{self, Command, CommandErr},
    reset::ResetValues,
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
pub struct CommandMap<'a>(HashMap<&'a str, CommandEntry>, String, Option<String>);

impl<'a> CommandMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(mut self, name: &'a str, help: Option<&str>, command: Box<dyn Command>) -> Self {
        self.0.insert(name, CommandEntry::new(command, help));
        self
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.1 = name;
        self
    }

    pub fn name(&self) -> &str {
        &self.1
    }

    pub fn set_lookup_backup(mut self, backup: Option<String>) -> Self {
        self.2 = backup;
        self
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
                } else if let Some(ref lookup_backup) = self.2 {
                    let Some(command) = self.0.get(lookup_backup.as_str()) else {
                        return Err(CommandErr::Lookup);
                    };

                    let mut forward_args = vec![name.into()];
                    forward_args.extend(args.iter().cloned());

                    command.command.borrow_mut().call(&forward_args)
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
        let bookmark_buffer = shared::Buffer::default();
        let category_buffer = shared::Buffer::default();
        let selected_bookmark = shared::Selected::default();
        let selected_category = shared::Selected::default();

        let reset_values = ResetValues {
            bookmark_buffer: bookmark_buffer.clone(),
            category_buffer: category_buffer.clone(),
            selected_category: selected_category.clone(),
            selected_bookmark: selected_bookmark.clone(),
        };

        use command::*;

        Self::new()
            .push("reset", None, reset::Reset::build(reset_values.clone()))
            .push(
                "category",
                None,
                category::Category::build(
                    "category".into(),
                    categories.clone(),
                    category_buffer.clone(),
                    selected_category.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "bookmark",
                None,
                bookmark::Bookmark::build(
                    "bookmark".into(),
                    bookmarks.clone(),
                    bookmark_buffer.clone(),
                    selected_bookmark.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "load",
                None,
                load::LoadAll::build(categories.clone(), bookmarks.clone(), reset_values.clone()),
            )
            .push(
                "save",
                None,
                save::SaveAll::build(categories.clone(), bookmarks.clone()),
            )
            .push("debug", None, Box::new(command_debug))
            .set_lookup_backup(Some("bookmark".into()))
    }
}
