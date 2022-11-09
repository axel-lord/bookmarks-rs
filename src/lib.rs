pub mod bookmark;
pub mod command;
pub mod command_map;
pub mod pattern_match;
pub mod token;

use crate::{command::build_command_map, command_map::CommandErr};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, error::Error, io, rc::Rc};

fn parse_command(line: &str) -> Option<(&str, Vec<String>)> {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    // make sure line is a command
    let Some(command_capture) = CMD_RE.captures(line) else {
        return None;
    };

    // if line is a command group 1 should always exist
    let command = command_capture
        .get(1)
        .expect("regex matched but required capture group did not (should never happen)");
    let command = command.as_str();

    // in case there are no arguments
    let Some(args) = command_capture.get(2) else {
        return Some((command, Vec::new()));
    };

    let args = args.as_str();

    let mut next_start = 0;
    let mut arg_vec = Vec::new();

    for capture in ARG_RE.captures_iter(&args) {
        // if capture exists so should whole capture
        let whole_capture = capture
            .get(0)
            .expect("could not get whole capture of regex (should never happen)");

        // used when parsing no quoted arguments
        let before = &args[next_start..whole_capture.start()];
        next_start = whole_capture.end();

        // iterate over quoted arguments appearing before captured end or quoted argument
        // and add them to arg vector
        arg_vec.extend(before.split_whitespace().map(String::from));

        // if a quoted argument was captured add it
        let Some(quoted_arg) = capture.get(1) else {
            continue;
        };

        arg_vec.push(quoted_arg.as_str().into());
    }

    Some((command, arg_vec))
}

pub fn run() -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let unsorted_bookmarks = Rc::new(RefCell::new(Vec::new()));
    let command_map = build_command_map(unsorted_bookmarks.clone());

    loop {
        println!("enter command:");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line from stdin");

        let command = command.trim();

        let Some((cmd, args)) = parse_command(&command) else {
            println!("could not parse \"{command}\"");
            continue;
        };

        if cmd == "exit" {
            break;
        }

        if let Err(err) = command_map.call(cmd, args) {
            match err {
                CommandErr::Lookup => println!("{cmd} is not a valid command"),
                CommandErr::Execution(s) => println!("failed to execute {cmd}: {s}"),
            }
        }
    }

    Ok(())
}
