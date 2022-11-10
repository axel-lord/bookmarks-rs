use std::{cell::RefCell, collections::HashMap, fmt::Debug};

pub enum CommandErr {
    Lookup,
    Execution(String),
}

pub trait Command: Debug {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr>;
}

impl<T: Debug> Command for T
where
    T: FnMut(&[String]) -> Result<(), CommandErr>,
{
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self(args)
    }
}

#[derive(Default, Debug)]
pub struct CommandMap<'a>(HashMap<&'a str, RefCell<Box<dyn Command>>>);

impl<'a> CommandMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, name: &'a str, command: Box<dyn Command>) {
        self.0.insert(name, RefCell::new(command));
    }

    pub fn call(&self, name: &str, args: &[String]) -> Result<(), CommandErr> {
        if let Some(command) = self.0.get(name) {
            command.borrow_mut().call(args)
        } else {
            Err(CommandErr::Lookup)
        }
    }
}
