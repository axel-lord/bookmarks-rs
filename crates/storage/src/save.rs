use std::io::{self, Write};

use crate::{Listed, Storeable};

/// Write the items of an iterator of [Listed].
///
/// # Errors
/// If a write fails.
pub fn save<'a, T>(writer: &mut impl Write, content: impl Iterator<Item = &'a T>) -> io::Result<()>
where
    T: 'a + Listed,
{
    writeln!(writer, "{}", T::TOKEN_BEGIN)?;

    for item in content.map(Storeable::to_line) {
        writeln!(writer, "{item}")?;
    }

    writeln!(writer, "{}", T::TOKEN_END)?;

    Ok(())
}
