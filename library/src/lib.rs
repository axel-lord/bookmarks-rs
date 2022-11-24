pub mod bookmark;
pub mod category;
pub mod command;
pub mod command_map;
pub mod info;
pub mod reset;
pub mod shared;

mod parse_command;

use crate::{
    bookmark::Bookmark, category::Category, command::CommandErr, command_map::CommandMap,
    info::Info, parse_command::parse_command, reset::ResetValues,
};
use command::Command;
use lazy_static::lazy_static;
use regex::Regex;
use std::io;

pub trait CommandBuilder {
    fn name(&self) -> &'static str;
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
        reset_values: ResetValues,
    ) -> Box<dyn Command>;
    fn help(&self) -> Option<&'static str> {
        None
    }
}

impl<F> CommandBuilder for (&'static str, F)
where
    F: FnMut(
        shared::BufferStorage<Bookmark>,
        shared::BufferStorage<Category>,
        shared::BufferStorage<Info>,
        ResetValues,
    ) -> Box<dyn Command>,
{
    fn name(&self) -> &'static str {
        self.0
    }
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
        reset_values: ResetValues,
    ) -> Box<dyn Command> {
        (self.1)(bookmarks, categories, infos, reset_values)
    }
}

impl<F> CommandBuilder for (&'static str, &'static str, F)
where
    F: FnMut(
        shared::BufferStorage<Bookmark>,
        shared::BufferStorage<Category>,
        shared::BufferStorage<Info>,
        ResetValues,
    ) -> Box<dyn Command>,
{
    fn name(&self) -> &'static str {
        self.0
    }
    fn help(&self) -> Option<&'static str> {
        Some(self.1)
    }
    fn build(
        &mut self,
        bookmarks: shared::BufferStorage<Bookmark>,
        categories: shared::BufferStorage<Category>,
        infos: shared::BufferStorage<Info>,
        reset_values: ResetValues,
    ) -> Box<dyn Command> {
        (self.2)(bookmarks, categories, infos, reset_values)
    }
}

pub fn run(
    init_commands: Option<String>,
    mut extended_commands: Vec<Box<dyn CommandBuilder>>,
) -> i32 {
    lazy_static! {
        static ref CMD_RE: Regex = Regex::new(r#"(\S+)\s*(.*)"#).unwrap();
        static ref ARG_RE: Regex = Regex::new(r#"\s*"(.*?)"\s*|$"#).unwrap();
    }

    let bookmarks = shared::BufferStorage::<Bookmark>::default();
    let categories = shared::BufferStorage::<Category>::default();
    let infos = shared::BufferStorage::<Info>::default();

    let reset_values = ResetValues {
        bookmark_buffer: bookmarks.buffer.clone(),
        category_buffer: categories.buffer.clone(),
        selected_category: categories.selected.clone(),
        selected_bookmark: bookmarks.selected.clone(),
    };

    let command_map = extended_commands
        .iter_mut()
        .fold(
            CommandMap::build(
                bookmarks.clone(),
                categories.clone(),
                infos.clone(),
                reset_values.clone(),
            ),
            |map, builder| {
                map.push(
                    builder.name(),
                    builder.help(),
                    builder.build(
                        bookmarks.clone(),
                        categories.clone(),
                        infos.clone(),
                        reset_values.clone(),
                    ),
                )
            },
        )
        .build();

    let eval_command = |command: &str, fatal_errors| -> Result<(), i32> {
        let command = command.trim();

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
