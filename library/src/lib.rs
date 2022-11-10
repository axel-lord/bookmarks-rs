pub mod bookmark;
pub mod category;
pub mod command;
pub mod command_map;
pub mod pattern_match;
pub mod token;

mod content_string;
mod parse_command;

pub use crate::content_string::ContentString;

use crate::{command::build_command_map, command_map::CommandErr, parse_command::parse_command};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, io, rc::Rc};

pub fn run() -> i32 {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let bookmarks = Rc::new(RefCell::new(Vec::new()));
    let categories = Rc::new(RefCell::new(Vec::new()));

    let command_map = build_command_map(bookmarks.clone(), categories.clone());

    loop {
        println!("enter command:");
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Err(err) => {
                eprintln!("failed to read from stdin: {:#?}", err);
                break 1;
            }
            Ok(0) => break 0,
            Ok(_) => (),
        }

        let command = command.trim();

        // let Some((cmd, args)) = parse_command(&command) else {
        let Some(args) = parse_command(&command) else {
            println!("could not parse \"{command}\"");
            continue;
        };

        let command = &args[0];

        if command == "exit" {
            break 0;
        }

        if let Err(err) = command_map.call(&command, &args[1..]) {
            match err {
                CommandErr::Lookup => println!("{command} is not a valid command"),
                CommandErr::Execution(s) => println!("failed to execute {command}: {s}"),
            }
        }
    }
}
