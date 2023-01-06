//! Crate use for definitions of command interface used by command line app.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

pub use bookmark_derive::Command;
use thiserror::Error;

/// Error type for commands.
#[derive(Debug, Clone, Error)]
pub enum CommandErr {
    /// When there has been an attempt to call a command that does not exist.
    #[error("command lookup failed")]
    Lookup,
    /// Something went wrong with the execution of the command.
    #[error("command execution failed: {0}")]
    Execution(String),
    /// A command was used incorrectly.
    #[error("incorrect usage: {0}")]
    Usage(String),
}

impl From<String> for CommandErr {
    fn from(value: String) -> Self {
        CommandErr::Execution(value)
    }
}

macro_rules! errors_to_execution_err {
    ($($err:ty),* $(,)?) => {
        $(
        impl From<$err> for CommandErr {
            fn from(value: $err) -> Self {
                CommandErr::Execution(format!("{value}"))
            }
        }
        )*
    };
}

errors_to_execution_err!(
    std::io::Error,
    bookmark_storage::ParseErr,
    bookmark_storage::PropertyErr
);

/// Trait for commands to implement.
pub trait Command {
    /// How the command is called, self is mut allowing for more dynamic commands.
    ///
    /// # Errors
    /// If the command was called with incorrect args, or something went wrong during the execution
    /// of the command.
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

/// Convenience function to generate a command error when the arguments to a command are not empty,
/// but should be.
///
/// # Errors
/// If args is not empty.
pub fn args_are_empty<T>(args: &[T]) -> Result<(), CommandErr> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(CommandErr::Usage(
            "command that takes no arguments called with arguments".into(),
        ))
    }
}
