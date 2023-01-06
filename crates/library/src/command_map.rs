//! Command maps are the main way to organizze commands, they can be nested due to themselves
//! implementing the [Command] trait.

mod bookmark;
mod category;
mod count;
mod info;
mod list;
mod load;
mod print;
mod push;
mod reset;
mod save;
mod select;
mod set;

use crate::{bookmark::Bookmark, category::Category, info::Info, shared};
use bookmark_command::{Command, CommandErr};
use std::{cell::RefCell, collections::HashMap, fmt::Debug};

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
            write!(f, "<{help}>")
        } else {
            write!(f, "<>")
        }
    }
}

/// Builder for [`CommandMap`].
#[derive(Debug, Default)]
pub struct Builder<'a> {
    commands: Vec<(&'a str, CommandEntry)>,
    name: String,
    fallback: Option<String>,
}

impl<'a> Builder<'a> {
    /// Create a new [Builder] same as [`Builder::default`].
    #[must_use]
    pub fn new() -> Self {
        Builder::default()
    }

    /// Push a [Command] to be used by the built [`CommandMap`].
    #[must_use]
    pub fn push(mut self, name: &'a str, help: Option<&str>, command: Box<dyn Command>) -> Self {
        self.commands.push((name, CommandEntry::new(command, help)));
        self
    }

    /// Set the name of the [`CommandMap`].
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Set a backup, if the the [Command] requested of the [`CommandMap`] does not exist it will
    /// forward it to this subcommand.
    #[must_use]
    pub fn lookup_backup(mut self, backup: Option<String>) -> Self {
        self.fallback = backup;
        self
    }

    /// Build a [`CommandMap`] consuming the [Builder].
    #[must_use]
    pub fn build(self) -> CommandMap<'a> {
        CommandMap {
            commands: self.commands.into_iter().collect(),
            name: self.name,
            fallback: self.fallback,
        }
    }
}

/// A map of commands to be called, the command map itself also implements [Command] allowing for
/// nested commands.
#[derive(Default, Debug)]
pub struct CommandMap<'a> {
    commands: HashMap<&'a str, CommandEntry>,
    name: String,
    fallback: Option<String>,
}

impl<'a> CommandMap<'a> {
    /// Gives the name of the [`CommandMap`].
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Call a command in the [`CommandMap`] and pass ite the given arguments, this is the main
    /// purpose of the [`CommandMap`].
    ///
    /// # Errors
    /// If the command fails in some way, or if it does not exist.
    pub fn call(&self, name: &str, args: &[String]) -> Result<&Self, CommandErr> {
        match name {
            "help" => match args.len() {
                0 => {
                    println!("available commands:");
                    for (command, entry) in &self.commands {
                        if let Some(ref help) = entry.help {
                            println!("- {command}, {help}");
                        } else {
                            println!("- {command}");
                        }
                    }
                    Ok(self)
                }
                1 => {
                    let command = &args[0];
                    if let Some(help) = self.help(command) {
                        println!("{help}");
                        Ok(self)
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
                    command.command.borrow_mut().call(args).map(|()| self)
                } else if let Some(ref lookup_backup) = self.fallback {
                    let Some(command) = self.commands.get(lookup_backup.as_str()) else {
                        return Err(CommandErr::Lookup);
                    };

                    let mut forward_args = vec![name.into()];
                    forward_args.extend(args.iter().cloned());

                    command
                        .command
                        .borrow_mut()
                        .call(&forward_args)
                        .map(|()| self)
                } else {
                    Err(CommandErr::Lookup)
                }
            }
        }
    }

    /// Get the hel message for a command, the message is given if the command exists and it has a
    /// help message.
    #[must_use]
    pub fn help(&self, name: &str) -> Option<String> {
        if name == "help" {
            Some(if self.name.is_empty() {
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
    /// Get a default config of the [`CommandMapBuilder`] this includes all default commands for
    /// handling bookmarks, categories, and info.
    pub fn default_config(
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
    ) -> Builder<'static> {
        Builder::new()
            .lookup_backup(Some("bookmark".into()))
            .push(
                "reset",
                None,
                reset::Reset::build(infos.clone(), categories.clone(), bookmarks.clone()),
            )
            .push(
                "category",
                None,
                category::build("category".into(), categories.clone(), bookmarks.clone()),
            )
            .push(
                "bookmark",
                None,
                bookmark::build("bookmark".into(), bookmarks.clone()),
            )
            .push(
                "info",
                None,
                info::build("info".into(), infos.clone(), categories.clone()),
            )
            .push(
                "load",
                None,
                load::LoadAll::build(categories.clone(), bookmarks.clone(), infos.clone()),
            )
            .push("save", None, save::All::build(infos, categories, bookmarks))
            .push(
                "debug",
                None,
                Box::new(|args: &[String]| {
                    println!("{args:#?}");
                    Ok(())
                }),
            )
    }
}

impl<'a> Command for CommandMap<'a> {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        CommandMap::call(
            self,
            args.get(0).ok_or_else(|| {
                CommandErr::Execution("needs to be called with a subcommand".into())
            })?,
            &args[1..],
        )
        .map(|_| ())
    }
}
