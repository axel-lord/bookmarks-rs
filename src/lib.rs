pub mod bookmark;
pub mod command_map;
pub mod token;

use crate::command_map::{CommandErr, CommandMap};
use bookmark::Bookmark;
use std::{cell::RefCell, error::Error, fs::File, io, rc::Rc};

pub fn build_command_map<'a>(bookmarks: Rc<RefCell<Vec<Bookmark<'a>>>>) -> CommandMap<'a> {
    let mut command_map = CommandMap::new();

    let list_instance = bookmarks.clone();
    command_map.push(
        "list",
        Box::new(move |args: String| {
            let args: Vec<_> = args.split(" ").filter(|s| s.len() != 0).collect();
            let bookarks = list_instance.clone();

            match args[..] {
                [count] => {
                    println!("list {count} bookmarks");
                    Ok(())
                }
                _ => {
                    println!("incorrect usage");
                    Err(CommandErr::Execution(
                        "incorrect number of arguments passed".into(),
                    ))
                }
            }
        }),
    );

    command_map
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let data = io::read_to_string(File::open("./bookmarks.txt")?)?;

    let unsorted_bookmarks = Rc::new(RefCell::new(
        data.lines()
            .enumerate()
            .skip_while(|(_, l)| !l.contains(token::UNSORTED_BEGIN))
            .skip(1)
            .take_while(|(_, l)| !l.contains(token::UNSORTED_END))
            .map(|(i, l)| Bookmark::with_str(l, Some(i)))
            .collect::<Result<Vec<_>, _>>()?,
    ));

    {
        let unsorted_bookmarks = unsorted_bookmarks.borrow();
        for l in unsorted_bookmarks.iter().rev().take(5).rev() {
            println!("Line |{l}|");
        }
    }

    let command_map = build_command_map(unsorted_bookmarks.clone());

    if let Err(CommandErr::Execution(msg)) = command_map.call("list", "5".into()) {
        println!("{msg}");
    }

    Ok(())
}
