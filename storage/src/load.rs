use crate::{Listed, ParseErr};
use std::io::{self, Read};

pub fn load<T, R>(reader: &mut R) -> Result<Vec<T>, ParseErr>
where
    T: Listed,
    R: Read,
{
    io::read_to_string(reader)?
        .lines()
        .enumerate()
        .skip_while(|(_, l)| !l.contains(T::TOKEN_BEGIN))
        .skip(1)
        .take_while(|(_, l)| !l.contains(T::TOKEN_END))
        .map(|(i, l)| T::with_str(l, Some(i)))
        .collect()
}
