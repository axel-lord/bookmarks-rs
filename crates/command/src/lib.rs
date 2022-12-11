pub use bookmark_derive::Command;

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

impl<T> From<T> for CommandErr
where
    T: std::error::Error,
{
    fn from(err: T) -> Self {
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

pub fn args_are_empty<T>(args: &[T]) -> Result<(), CommandErr> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(CommandErr::Usage(
            "command that takes no arguments called with arguments".into(),
        ))
    }
}
