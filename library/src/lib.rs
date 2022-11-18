pub mod bookmark;
pub mod category;
pub mod command;
pub mod command_map;
pub mod info;
pub mod reset;
pub mod shared;

mod parse_command;

use crate::{command::CommandErr, command_map::CommandMap, parse_command::parse_command};
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

pub fn run(init_commands: Option<String>) -> i32 {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let bookmarks = shared::Bookmarks::default();
    let categories = shared::Categroies::default();

    let command_map = CommandMap::build(bookmarks.clone(), categories.clone());

    let eval_command = |command: &str, fatal_errors| -> Result<(), i32> {
        let command = command.trim();

        // let Some((cmd, args)) = parse_command(&command) else {
        let Some(args) = parse_command(&command) else {
            println!("could not parse \"{command}\"");
            return Ok(());
        };

        let command = &args[0];

        if command == "exit" {
            return Err(0);
        }

        if let Err(err) = command_map.call(&command, &args[1..]) {
            match err {
                CommandErr::Usage(ref msg) => {
                    println!("incorrect usage: {msg}");
                    if let Some(help) = command_map.help(&command) {
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
            match eval_command(&command, true) {
                Err(0) => break,
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
