//! Library for handling bookmarks.

#![warn(
    missing_copy_implementations, 
    // missing_docs,
    // clippy::missing_errors_doc, 
    // clippy::missing_panics_doc, 
    // clippy::missing_safety_doc, 
    rustdoc::missing_crate_level_docs
)]

pub mod bookmark;
pub mod category;
pub mod command;
pub mod command_map;
pub mod container;
pub mod info;
pub mod shared;
pub mod command_factory;

mod parse_command;

use crate::{
    bookmark::Bookmark, category::Category, command_map::CommandMap,
    info::Info, parse_command::parse_command,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

pub fn run(
    init_commands: Option<String>,
    mut extended_commands: Vec<Box<dyn command_factory::CommandFactory>>,
) -> i32 {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let bookmarks = shared::BufferStorage::<Bookmark>::default();
    let categories = shared::BufferStorage::<Category>::default();
    let infos = shared::BufferStorage::<Info>::default();

    let command_map = extended_commands
        .iter_mut()
        .fold(
            CommandMap::default_config(
                bookmarks.clone(),
                categories.clone(),
                infos.clone(),
            ),
            |map, builder| {
                map.push(
                    builder.name(),
                    builder.help(),
                    builder.build(
                        bookmarks.clone(),
                        categories.clone(),
                        infos.clone(),
                    ),
                )
            },
        )
        .build();

    let eval_command = |command: &str, fatal_errors| -> Result<(), i32> {
        let command = command.trim();

        let Some(args) = parse_command(command) else {
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

        match io::stdin().read_line(&mut command) {
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
