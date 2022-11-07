pub mod bookmark;
pub mod command_map;
pub mod token;

use crate::command_map::{CommandErr, CommandMap};
use bookmark::Bookmark;
use std::{cell::RefCell, error::Error, fs::File, io, rc::Rc};

pub fn build_command_map(bookmarks: Rc<RefCell<Vec<Bookmark>>>) -> CommandMap<'static> {
    let mut command_map = CommandMap::new();

    {
        let bookmarks = bookmarks.clone();
        command_map.push(
            "list",
            Box::new(move |args: String| {
                let args: Vec<_> = args.split(" ").filter(|s| s.len() != 0).collect();

                match args[..] {
                    [] => {
                        for bookmark in bookmarks.borrow().iter() {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    [count] => {
                        println!("list {count} bookmarks");
                        let count = match count.parse() {
                            Ok(c) => c,
                            Err(_) => {
                                return Err(CommandErr::Execution(format!(
                                    "could not parse {count} as a bookmark count"
                                )))
                            }
                        };
                        for bookmark in bookmarks.borrow().iter().take(count) {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    [count, from] => {
                        println!("list {count} bookmarks");
                        let count = match count.parse() {
                            Ok(c) => c,
                            Err(_) => {
                                return Err(CommandErr::Execution(format!(
                                    "could not parse {count} as a bookmark count"
                                )))
                            }
                        };
                        let from = match from.parse() {
                            Ok(f) => f,
                            Err(_) => {
                                return Err(CommandErr::Execution(format!(
                                    "could not parse {count} as a bookmark index"
                                )))
                            }
                        };
                        for bookmark in bookmarks.borrow().iter().skip(from).take(count) {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    _ => Err(CommandErr::Execution("Usage: list [COUNT] [FROM]".into())),
                }
            }),
        );
    }

    command_map
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let unsorted_bookmarks = Rc::new(RefCell::new(
        io::read_to_string(File::open("./bookmarks.txt")?)?
            .lines()
            .enumerate()
            .skip_while(|(_, l)| !l.contains(token::UNSORTED_BEGIN))
            .skip(1)
            .take_while(|(_, l)| !l.contains(token::UNSORTED_END))
            .map(|(i, l)| Bookmark::with_str(l, Some(i)))
            .collect::<Result<Vec<_>, _>>()?,
    ));

    let command_map = build_command_map(unsorted_bookmarks.clone());

    if let Err(CommandErr::Execution(msg)) = command_map.call("list", "10".into()) {
        println!("{msg}");
    }

    println!("Bookmark count {}", unsorted_bookmarks.borrow().len());

    Ok(())
}
