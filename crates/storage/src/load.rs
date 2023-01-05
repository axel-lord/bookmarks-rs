use crate::{Listed, ParseErr};
use std::io::{self, BufRead, BufReader, Read};

/// Load occurances of a [Listed] type from a [Read].
///
/// # Errors
/// If a line cannot be parsed a [`ParseErr`] will be issued.
pub fn load<T>(reader: &mut impl Read) -> Result<Vec<T>, ParseErr>
where
    T: Listed,
{
    let reader = BufReader::new(reader);
    from(reader.lines().enumerate())
}

/// Load occurances of a [Listed] type from an enumerated iterator of line parses.
///
/// # Errors
/// If any of the line parses in the passed iterator is an error, eErr] will
/// be returned.
pub fn from<T>(
    mut reader: impl Iterator<Item = (usize, io::Result<String>)>,
) -> Result<Vec<T>, ParseErr>
where
    T: Listed,
{
    for (_, result) in reader.by_ref() {
        if result? == T::TOKEN_BEGIN {
            break;
        }
    }

    let mut out = Vec::new();

    for (i, result) in reader.by_ref() {
        let line = result?;
        if line == T::TOKEN_END {
            break;
        }

        out.push(T::with_string(line, Some(i))?);
    }

    Ok(out)
}
