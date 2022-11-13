use std::{cell::RefCell, collections::HashMap, error::Error, fmt::Debug};

#[derive(Debug, Clone)]
pub enum CommandErr {
    Lookup,
    Execution(String),
}

impl std::fmt::Display for CommandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandErr::Lookup => write!(f, "command lookup failed"),
            CommandErr::Execution(ref msg) => write!(f, "command execution failed: {}", msg),
        }
    }
}

impl Error for CommandErr {}

impl From<bookmark_storage::ParseErr> for CommandErr {
    fn from(err: bookmark_storage::ParseErr) -> Self {
        Self::Execution(format!("{err}"))
    }
}

pub trait Command {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr>;
}

impl<T> Command for T
where
    T: FnMut(&[String]) -> Result<(), CommandErr>,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self(args)
    }
}

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
