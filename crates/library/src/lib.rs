//! Library for handling bookmarks.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::missing_crate_level_docs
)]

pub mod command_map;
pub mod container;
pub mod token;

/// More easily use shared [container::BufferStorage].
pub mod shared {
    /// Since a lot of commands need access to reference counted storag, this type is used as a
    /// conveniance to simplify their signatures.
    pub type BufferStorage<T> =
        std::sync::Arc<std::sync::RwLock<super::container::BufferStorage<T>>>;
}

use std::collections::HashMap;

pub use bookmark::Bookmark;
pub use category::{Category, IdentifierContainer, IdentifierErr};
pub use command_factory::CommandFactory;
pub use info::Info;

mod bookmark;
mod category;
mod command_factory;
mod info;
mod parse_command;

use thiserror::Error;

/// Enum for Graph related errors.
#[derive(Debug, Error, Clone, Copy)]
pub enum GraphError {
    /// Value used when access to an invalid node is attempted.
    #[error("Invalid Node Index (value {value:?}, max {max:?})")]
    InvalidNode {
        /// The invalid node.
        value: usize,
        /// The max value a node can be.
        max: usize,
    },
}

/// Used to represent a graph.
pub struct Graph {
    data: Box<[bool]>,
    node_count: usize,
}

impl Graph {
    /// Check if there is an edge between give nodes.
    ///
    /// # Errors
    /// If a node index is invalid.
    pub fn is_edge(&self, from: usize, to: usize) -> Result<bool, GraphError> {
        if from > self.node_count {
            Err(GraphError::InvalidNode {
                value: from,
                max: self.node_count,
            })
        } else if to > self.node_count {
            Err(GraphError::InvalidNode {
                value: to,
                max: self.node_count,
            })
        } else {
            Ok(self.data[from * self.node_count + to])
        }
    }

    /// Set and edge between two nodes.
    ///
    /// # Errors
    /// If to or from is not a node in graph.
    pub fn set_edge(&mut self, from: usize, to: usize, value: bool) -> Result<(), GraphError> {
        if from > self.node_count {
            Err(GraphError::InvalidNode {
                value: from,
                max: self.node_count,
            })
        } else if to > self.node_count {
            Err(GraphError::InvalidNode {
                value: to,
                max: self.node_count,
            })
        } else {
            self.data[from * self.node_count + to] = value;
            Ok(())
        }
    }
}

impl From<&[Category]> for Graph {
    fn from(value: &[Category]) -> Self {
        let mut graph = Self {
            data: vec![false; value.len() * value.len()].into_boxed_slice(),
            node_count: value.len(),
        };

        let mut cat_map = HashMap::new();
        for (i, category) in value.iter().enumerate() {
            cat_map.insert(Box::new(category.id()), i);
        }

        for (i, category) in value.iter().enumerate() {
            for child in category.subcategories() {
                let Some(sub_i) = cat_map.get(&Box::new(child)) else {
                    continue;
                };

                if let Err(err) = graph.set_edge(i, *sub_i, true) {
                    eprintln!("{err}");
                };
            }
        }

        graph
    }
}

use regex::Regex;

/// Run a command line bookmark manager.
pub fn run(
    init_commands: Option<String>,
    mut extended_commands: Vec<Box<dyn command_factory::CommandFactory>>,
) -> i32 {
    lazy_static::lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let bookmarks = shared::BufferStorage::<bookmark::Bookmark>::default();
    let categories = shared::BufferStorage::<category::Category>::default();
    let infos = shared::BufferStorage::<info::Info>::default();

    let command_map = extended_commands
        .iter_mut()
        .fold(
            command_map::CommandMap::default_config(
                bookmarks.clone(),
                categories.clone(),
                infos.clone(),
            ),
            |map, builder| {
                map.push(
                    builder.name(),
                    builder.help(),
                    builder.build(bookmarks.clone(), categories.clone(), infos.clone()),
                )
            },
        )
        .build();

    let eval_command = |command: &str, fatal_errors| -> Result<(), i32> {
        let command = command.trim();

        let Some(args) = parse_command::parse_command(command) else {
            println!("could not parse \"{command}\"");
            return Ok(());
        };

        let command = &args[0];

        if command == "exit" {
            return Err(0);
        }

        if let Err(err) = command_map.call(command, &args[1..]) {
            match err {
                bookmark_command::CommandErr::Usage(ref msg) => {
                    println!("incorrect usage: {msg}");
                    if let Some(help) = command_map.help(command) {
                        println!("{help}");
                    };
                }
                err => println!("{err}"),
            }

            if fatal_errors {
                return Err(1);
            }
        }
        Ok(())
    };

    if let Some(init_commands) = init_commands {
        for command in init_commands.lines() {
            match eval_command(command, true) {
                Err(0) => return 0,
                Err(code) => {
                    println!("error running init commands");
                    return code;
                }
                Ok(_) => (),
            }
        }
    }

    let mut command = String::new();
    loop {
        command.clear();
        println!("enter command:");

        match std::io::stdin().read_line(&mut command) {
            Err(err) => {
                eprintln!("failed to read from stdin: {}", err);
                break 1;
            }
            Ok(0) => break 0,
            Ok(_) => (),
        }

        if let Err(code) = eval_command(&command, false) {
            break code;
        }
    }
}
