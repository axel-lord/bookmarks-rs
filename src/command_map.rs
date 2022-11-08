use std::{cell::RefCell, collections::HashMap};

pub enum CommandErr {
    Lookup,
    Execution(String),
}

pub trait Command {
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr>;
}

impl<T> Command for T
where
    T: FnMut(Vec<String>) -> Result<(), CommandErr>,
{
    fn call(&mut self, args: Vec<String>) -> Result<(), CommandErr> {
        self(args)
    }
}

#[derive(Default)]
pub struct CommandMap<'a>(HashMap<&'a str, RefCell<Box<dyn Command>>>);

impl<'a> CommandMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, name: &'a str, command: Box<dyn Command>) {
        self.0.insert(name, RefCell::new(command));
    }

    pub fn call(&self, name: &str, args: Vec<String>) -> Result<(), CommandErr> {
        if let Some(command) = self.0.get(name) {
            command.borrow_mut().call(args)
        } else {
            Err(CommandErr::Lookup)
        }
    }
}
