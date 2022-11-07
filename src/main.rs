use std::{error::Error, fs::File, io};

fn main() -> Result<(), Box<dyn Error>> {
    let data = io::read_to_string(File::open("./bookmarks.txt")?)?;

    let unsorted_bookmark_lines = data
        .lines()
        .skip_while(|l| !l.contains("#UNSORTED_BEGIN"))
        .skip(1)
        .take_while(|l| !l.contains("#UNSORTED_END"));

    for l in unsorted_bookmark_lines {
        println!("Line |{l}|");
    }

    Ok(())
}
