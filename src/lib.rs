pub mod bookmark;
pub mod command_map;
pub mod pattern_match;
pub mod token;

use crate::command_map::{CommandErr, CommandMap};
use bookmark::Bookmark;
use std::{cell::RefCell, error::Error, fs::File, io, ops::Range, rc::Rc};

fn wrap_if_negative(number: isize, max: usize) -> Result<usize, CommandErr> {
    if number.abs() as usize > max {
        return Err(CommandErr::Execution(format!(
            "number {number} larger than max value {max}"
        )));
    }

    if number >= 0 {
        Ok(number as usize)
    } else {
        Ok(max - number.abs() as usize)
    }
}

fn get_bookmark_iter<'a>(
    bookmarks: &'a Vec<Bookmark>,
    buffer: &'a Vec<Range<usize>>,
) -> impl Iterator<Item = (usize, &'a Bookmark)> {
    buffer
        .iter()
        .map(|r| r.clone().into_iter().map(|i| (i, &bookmarks[i])))
        .flatten()
}

fn buffer_length(buffer: &Vec<Range<usize>>) -> usize {
    buffer.iter().map(Range::len).fold(0, |acc, x| acc + x)
}

fn bookmark_filter_iter<'a, F>(
    bookmarks: impl 'a + Iterator<Item = (usize, &'a Bookmark)>,
    mut condition: F,
) -> impl Iterator<Item = Range<usize>> + 'a
where
    F: 'a + FnMut(&Bookmark) -> bool,
{
    bookmarks.filter_map(move |(i, bookmark)| {
        if condition(bookmark) {
            Some(i..i + 1)
        } else {
            None
        }
    })
}

fn get_filtered_bookmarks<'a, I, F>(bookmarks: I, condition: F) -> Vec<Range<usize>>
where
    I: 'a + Iterator<Item = (usize, &'a Bookmark)>,
    F: 'a + FnMut(&Bookmark) -> bool,
{
    bookmark_filter_iter(bookmarks, condition).collect()
}

pub fn build_command_map(bookmarks: Rc<RefCell<Vec<Bookmark>>>) -> CommandMap<'static> {
    let mut command_map = CommandMap::new();
    let buffer = Rc::new(RefCell::new(vec![(0..bookmarks.borrow().len())]));

    {
        let bookmarks = bookmarks.clone();
        let buffer = buffer.clone();
        command_map.push(
            "list",
            Box::new(move |args: Vec<String>| {
                let bookmarks = bookmarks.borrow();
                let buffer = buffer.borrow();

                let bookmark_iter = get_bookmark_iter(&bookmarks, &buffer);
                match &args[..] {
                    [] => {
                        println!("listing all bookmarks");
                        for (_, bookmark) in bookmark_iter {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    [count] => {
                        println!("listing {count} bookmarks");
                        let count = match count.parse() {
                            Ok(c) => c,
                            Err(_) => {
                                return Err(CommandErr::Execution(format!(
                                    "could not parse {count} as a bookmark count"
                                )))
                            }
                        };
                        for (_, bookmark) in bookmark_iter.take(count) {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    [count, from] => {
                        println!("listing {count} bookmarks starting at index {from}");
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

                        let from = wrap_if_negative(from, buffer_length(&buffer))?;

                        for (_, bookmark) in bookmark_iter.skip(from).take(count) {
                            println!("{bookmark}");
                        }
                        Ok(())
                    }
                    _ => Err(CommandErr::Execution("Usage: list [COUNT] [FROM]".into())),
                }
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "filter",
            Box::new(move |args: Vec<String>| {
                if args.is_empty() {
                    return Err(CommandErr::Execution(
                        "filter needs one or more arguments".into(),
                    ));
                }

                let filtered = get_filtered_bookmarks(
                    get_bookmark_iter(&bookmarks.borrow(), &buffer.borrow()),
                    |bookmark| args.iter().all(|arg| bookmark.url().contains(arg)),
                );

                buffer.replace(filtered);

                Ok(())
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "filter-inv",
            Box::new(move |args: Vec<String>| {
                if args.is_empty() {
                    return Err(CommandErr::Execution(
                        "filter-inv needs one or more arguments".into(),
                    ));
                }

                let filtered = get_filtered_bookmarks(
                    get_bookmark_iter(&bookmarks.borrow(), &buffer.borrow()),
                    |bookmark| !args.iter().any(|arg| bookmark.url().contains(arg)),
                );

                buffer.replace(filtered);

                Ok(())
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "regex",
            Box::new(move |args: Vec<String>| {
                if args.is_empty() {
                    return Err(CommandErr::Execution("regex needs a pattern".into()));
                }

                let pattern = args.join(" ");
                let Ok(re) = regex::Regex::new(&pattern) else {
                    return Err(CommandErr::Execution(format!("invalid pattern /{pattern}/")));
                };

                let filtered = get_filtered_bookmarks(
                    get_bookmark_iter(&bookmarks.borrow(), &buffer.borrow()),
                    |bookmark| re.is_match(bookmark.url()),
                );

                buffer.replace(filtered);

                Ok(())
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "regex-inv",
            Box::new(move |args: Vec<String>| {
                if args.is_empty() {
                    return Err(CommandErr::Execution("regex-inv needs a pattern".into()));
                }

                let pattern = args.join(" ");
                let Ok(re) = regex::Regex::new(&pattern) else {
                    return Err(CommandErr::Execution(format!("invalid pattern /{pattern}/")));
                };

                let filtered = get_filtered_bookmarks(
                    get_bookmark_iter(&bookmarks.borrow(), &buffer.borrow()),
                    |bookmark| !re.is_match(bookmark.url()),
                );

                buffer.replace(filtered);

                Ok(())
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "count",
            Box::new(move |args: Vec<String>| {
                if !args.is_empty() {
                    return Err(CommandErr::Execution(
                        "count should be used without any arguments".into(),
                    ));
                }

                let total = bookmarks.borrow().len();
                let in_buffer = buffer_length(&buffer.borrow());

                println!("total: {total}, in buffer: {in_buffer}");

                Ok(())
            }),
        );
    }

    {
        let (buffer, bookmarks) = (buffer.clone(), bookmarks.clone());
        command_map.push(
            "reset",
            Box::new(move |args: Vec<String>| {
                if !args.is_empty() {
                    return Err(CommandErr::Execution(
                        "reset should be used without any arguments".into(),
                    ));
                }

                buffer.replace(vec![(0..bookmarks.borrow().len())]);

                Ok(())
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
            .map(|(i, l)| Bookmark::with_str(l.into(), Some(i)))
            .collect::<Result<Vec<_>, _>>()?,
    ));

    let command_map = build_command_map(unsorted_bookmarks.clone());

    loop {
        println!("enter command:");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("failed to read line from stdin");

        let mut command = command.split_whitespace();
        if let Some(cmd) = command.next() {
            if cmd == "exit" {
                break;
            }

            let args = command.map(String::from).collect();

            if let Err(err) = command_map.call(cmd, args) {
                match err {
                    CommandErr::Lookup => println!("{cmd} is not a valid command"),
                    CommandErr::Execution(s) => println!("failed to execute {cmd}: {s}"),
                }
            }
        }
    }

    Ok(())
}
