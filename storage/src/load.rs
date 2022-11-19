use crate::{Listed, ParseErr};
use std::io::{self, BufRead, BufReader, Read};

pub fn load<T, R>(reader: &mut R) -> Result<Vec<T>, ParseErr>
where
    T: Listed,
    R: Read,
{
    let reader = BufReader::new(reader);
    load_from(reader.lines().enumerate())
}

pub fn load_from<T>(
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
