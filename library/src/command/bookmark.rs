use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    bookmark,
    command_map::{Command, CommandErr, CommandMap},
};

#[derive(Debug, Default)]
pub struct Bookmark {
    command_map: CommandMap<'static>,
}

impl Bookmark {
    pub fn build(
        _bookmarks: Rc<RefCell<Vec<bookmark::Bookmark>>>,
        _buffer: Rc<RefCell<Vec<Range<usize>>>>,
    ) -> Box<Self> {
        let subcommand: Box<Self> = Default::default();

        subcommand
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
