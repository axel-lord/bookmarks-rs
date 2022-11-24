use std::{cell::RefCell, collections::HashMap, fmt::Debug, iter::FromIterator};

use crate::{
    bookmark::Bookmark,
    category::Category,
    command::{self, Command, CommandErr},
    info::Info,
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

#[derive(Debug, Default)]
pub struct CommandMapBuilder<'a> {
    commands: Vec<(&'a str, CommandEntry)>,
    name: String,
    fallback: Option<String>,
}

impl<'a> CommandMapBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(mut self, name: &'a str, help: Option<&str>, command: Box<dyn Command>) -> Self {
        self.commands.push((name, CommandEntry::new(command, help)));
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn lookup_backup(mut self, backup: Option<String>) -> Self {
        self.fallback = backup;
        self
    }

    pub fn build(self) -> CommandMap<'a> {
        CommandMap {
            commands: HashMap::from_iter(self.commands.into_iter()),
            name: self.name,
            fallback: self.fallback,
        }
    }
}

#[derive(Default, Debug)]
pub struct CommandMap<'a> {
    commands: HashMap<&'a str, CommandEntry>,
    name: String,
    fallback: Option<String>,
}

impl<'a> CommandMap<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn call(&self, name: &str, args: &[String]) -> Result<(), CommandErr> {
        match name {
            "help" => match args.len() {
                0 => {
                    println!("available commands:");
                    for (command, entry) in self.commands.iter() {
                        if let Some(ref help) = entry.help {
                            println!("- {command}, {help}");
                        } else {
                            println!("- {command}");
                        }
                    }
                    Ok(())
                }
                1 => {
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
                _ => Err(CommandErr::Usage(
                    "help called with incorrect number of arguments".into(),
                )),
            },
            _ => {
                if let Some(command) = self.commands.get(name) {
                    command.command.borrow_mut().call(args)
                } else if let Some(ref lookup_backup) = self.fallback {
                    let Some(command) = self.commands.get(lookup_backup.as_str()) else {
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
            Some(if self.name.len() == 0 {
                "show help for a command\nusage: help COMMAND".into()
            } else {
                format!("show help for a command\nusage: {} help COMMAND", self.name)
            })
        } else {
            self.commands.get(name)?.help.clone()
        }
    }
}

impl CommandMap<'static> {
    pub fn build(
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
        reset_values: ResetValues,
    ) -> CommandMapBuilder<'static> {
        use command::*;

        CommandMapBuilder::new()
            .push("reset", None, reset::Reset::build(reset_values.clone()))
            .push(
                "category",
                None,
                category::build(
                    "category".into(),
                    categories.clone(),
                    bookmarks.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "bookmark",
                None,
                bookmark::build(
                    "bookmark".into(),
                    bookmarks.storage.clone(),
                    bookmarks.buffer.clone(),
                    bookmarks.selected.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "info",
                None,
                info::build(
                    "info".into(),
                    reset_values.clone(),
                    infos.storage,
                    infos.buffer,
                    infos.selected,
                ),
            )
            .push(
                "load",
                None,
                load::LoadAll::build(
                    categories.storage.clone(),
                    bookmarks.storage.clone(),
                    reset_values.clone(),
                ),
            )
            .push(
                "save",
                None,
                save::SaveAll::build(categories.storage.clone(), bookmarks.storage.clone()),
            )
            .push("debug", None, Box::new(command_debug))
            .lookup_backup(Some("bookmark".into()))
    }
}
