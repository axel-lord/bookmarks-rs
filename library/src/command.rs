pub mod bookmark;
pub mod category;
pub mod list;
pub mod load;
pub mod reset;
pub mod save;
pub mod select;

use std::error::Error;

pub fn command_debug(args: &[String]) -> Result<(), CommandErr> {
    println!("{:#?}", args);
    Ok(())
}

#[derive(Debug, Clone)]
pub enum CommandErr {
    Lookup,
    Execution(String),
    Usage(String),
}

impl std::fmt::Display for CommandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandErr::Lookup => write!(f, "command lookup failed"),
            CommandErr::Execution(ref msg) => write!(f, "command execution failed: {}", msg),
            CommandErr::Usage(ref msg) => write!(f, "incorrect usage: {}", msg),
        }
    }
}

impl Error for CommandErr {}

impl From<bookmark_storage::ParseErr> for CommandErr {
    fn from(err: bookmark_storage::ParseErr) -> Self {
        Self::Execution(format!("{err}"))
    }
}

impl From<bookmark_storage::PropertyErr> for CommandErr {
    fn from(err: bookmark_storage::PropertyErr) -> Self {
        Self::Execution(format!("{err}"))
    }
}

impl From<std::io::Error> for CommandErr {
    fn from(err: std::io::Error) -> Self {
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
