use std::{
    fs::File,
    io::{self, prelude::*, BufWriter},
};

use crate::Listed;

pub fn save<'a, T, I>(path: &str, content: I) -> io::Result<()>
where
    T: 'a + Listed,
    I: Iterator<Item = &'a T>,
{
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", T::token_begin())?;

    for item in content.map(|item| item.to_line()) {
        writeln!(writer, "{item}")?;
    }

    writeln!(writer, "{}", T::token_end())?;

    Ok(())
}
