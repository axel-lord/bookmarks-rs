use crate::{ParseErr, Section, Storeable};
use std::{fs::File, io};

pub fn load<T: Storeable + Section>(path: &str) -> Result<Vec<T>, ParseErr> {
    io::read_to_string(
        File::open(path)
            .map_err(|err| ParseErr::Other(format!("could not open {}: {}", path, err)))?,
    )
    .map_err(|err| ParseErr::Other(format!("failed to read {}: {}", path, err)))?
    .lines()
    .enumerate()
    .skip_while(|(_, l)| !l.contains(T::token_begin()))
    .skip(1)
    .take_while(|(_, l)| !l.contains(T::token_end()))
    .map(|(i, l)| T::with_str(l, Some(i)))
    .collect()
}
