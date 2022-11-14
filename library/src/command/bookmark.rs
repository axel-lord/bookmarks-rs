pub mod count;
pub mod filter;
pub mod list;
pub mod regex;
pub mod save;

use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark,
    command::{load::Load, Command, CommandErr},
    command_map::CommandMap,
};

#[derive(Debug, Default)]
pub struct Bookmark {
    command_map: CommandMap<'static>,
}

impl Bookmark {
    pub fn build(
        name: String,
        bookmarks: Rc<RefCell<Vec<bookmark::Bookmark>>>,
        buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        let mut subcommand = CommandMap::new();
        subcommand.set_name(name);

        subcommand.push(
            "list",
            None,
            list::List::build(bookmarks.clone(), buffer.clone()),
        );

        subcommand.push(
            "filter",
            None,
            filter::Filter::build(bookmarks.clone(), buffer.clone()),
        );

        subcommand.push(
            "filter-inv",
            None,
            filter::FilterInv::build(bookmarks.clone(), buffer.clone()),
        );

        subcommand.push(
            "regex",
            None,
            regex::Regex::build(bookmarks.clone(), buffer.clone()),
        );

        subcommand.push(
            "regex-inv",
            None,
            regex::RegexInv::build(bookmarks.clone(), buffer.clone()),
        );

        subcommand.push(
            "count",
            None,
            count::Count::build(bookmarks.clone(), buffer.clone()),
        );
        subcommand.push("load", None, Load::build(bookmarks.clone()));

        subcommand.push(
            "save",
            None,
            save::Save::build(bookmarks.clone(), buffer.clone()),
        );

        Box::new(Self {
            command_map: subcommand,
        })
    }
}

impl Command for Bookmark {
    fn call(&mut self, args: &[String]) -> Result<(), CommandErr> {
        self.command_map.call(
            &args.get(0).ok_or_else(|| {
                CommandErr::Execution("category needs to be called with a subcommand".into())
            })?,
            &args[1..],
        )
    }
}
