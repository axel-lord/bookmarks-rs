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
    let Some(m) = CMD_RE.captures(line) else {
        return None;
    };

    let Some(cmd) = m.get(1) else {
        return None;
    };

    let cmd = cmd.as_str();

    let args = if let Some(args) = m.get(2) {
        let args = args.as_str();
        let mut next_start = 0;
        let mut out = Vec::new();
        for cap in ARG_RE.captures_iter(&args) {
            let whole = cap.get(0).unwrap();

            for arg in args[next_start..whole.start()].split_whitespace() {
                out.push(arg.into());
            }

            next_start = whole.end();

            if let Some(quote) = cap.get(1) {
                out.push(quote.as_str().into())
            };
        }
        out
    } else {
        Vec::new()
    };

    Some((cmd, args))
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

        let Some((cmd, args)) = parse_command(&command) else {
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
