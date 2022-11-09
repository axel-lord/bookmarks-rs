pub mod bookmark;
pub mod category;
pub mod command;
pub mod command_map;
pub mod pattern_match;
pub mod token;

use crate::{command::build_command_map, command_map::CommandErr};
use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, error::Error, io, ops::Range, rc::Rc};

#[derive(Debug, Clone)]
pub enum ContentString {
    AppendedTo(String),
    UnappendedTo(String),
}

impl ContentString {
    pub fn take_any(self) -> String {
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    pub fn ref_any(&self) -> &str {
        match self {
            Self::AppendedTo(s) | Self::UnappendedTo(s) => s,
        }
    }

    pub fn is_appended_to(&self) -> bool {
        match self {
            Self::UnappendedTo(_) => false,
            Self::AppendedTo(_) => true,
        }
    }

    pub fn append(self, content: &str) -> (Self, Range<usize>) {
        let mut existing = self.take_any();

        let begin = existing.len();
        existing += content;
        let end = existing.len();

        (ContentString::AppendedTo(existing), begin..end)
    }
}

fn parse_command(line: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let mut next_start = 0;
    let mut arg_vec = Vec::new();

    for capture in ARG_RE.captures_iter(&line) {
        // if capture exists so should whole capture
        let whole_capture = capture
            .get(0)
            .expect("could not get whole capture of regex (should never happen)");

        // used when parsing no quoted arguments
        let before = &line[next_start..whole_capture.start()];
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

    Some(arg_vec)
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

        // let Some((cmd, args)) = parse_command(&command) else {
        let Some(args) = parse_command(&command) else {
            println!("could not parse \"{command}\"");
            continue;
        };

        let command = &args[0];

        if command == "exit" {
            break;
        }

        if let Err(err) = command_map.call(&command, &args[1..]) {
            match err {
                CommandErr::Lookup => println!("{command} is not a valid command"),
                CommandErr::Execution(s) => println!("failed to execute {command}: {s}"),
            }
        }
    }

    Ok(())
}
