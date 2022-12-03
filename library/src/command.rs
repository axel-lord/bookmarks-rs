pub mod bookmark;
pub mod category;
pub mod count;
pub mod info;
pub mod list;
pub mod load;
pub mod print;
pub mod push;
pub mod reset;
pub mod save;
pub mod select;
pub mod set;

use crate::command_map::CommandMap;
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

#[derive(Debug, Clone)]
struct CommandErrErr<'a>(&'a CommandErr);

// macro_rules! trivial_command_err_from {
//     ($($other:ty),*) => {
//         $(
//             impl From<$other> for CommandErr {
//                 fn from(err: $other) -> Self {
//                     Self::Execution(format!("{err}"))
//                 }
//             }
//         )*
//     };
// }

// trivial_command_err_from!(
//     bookmark_storage::ParseErr,
//     bookmark_storage::PropertyErr,
//     IdentifierErr,
//     std::io::Error,
//     GetSelectedErr
// );

impl<T> From<T> for CommandErr
where
    T: Error,
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

impl<'a> Command for CommandMap<'a> {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        CommandMap::call(
            self,
            args.get(0).ok_or_else(|| {
                CommandErr::Execution("needs to be called with a subcommand".into())
            })?,
            &args[1..],
        )
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
