use std::io::{self, Write};

use crate::Listed;

pub fn save<'a, T, I, W>(writer: &mut W, content: I) -> io::Result<()>
where
    T: 'a + Listed,
    I: Iterator<Item = &'a T>,
    W: Write,
{
    writeln!(writer, "{}", T::token_begin())?;

    for item in content.map(|item| item.to_line()) {
        writeln!(writer, "{item}")?;
    }

    writeln!(writer, "{}", T::token_end())?;

    Ok(())
}
