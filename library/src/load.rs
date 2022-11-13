use std::{fs::File, io};

pub use bookmark_storage::ParseErr;

pub fn load<T>(
    path: &str,
    start_line: &str,
    end_line: &str,
    mut parser: impl FnMut(&str, Option<usize>) -> Result<T, ParseErr>,
) -> Result<Vec<T>, ParseErr> {
    io::read_to_string(
        File::open(path)
            .map_err(|err| ParseErr::Other(format!("could not open {}: {}", path, err)))?,
    )
    .map_err(|err| ParseErr::Other(format!("failed to read {}: {}", path, err)))?
    .lines()
    .enumerate()
    .skip_while(|(_, l)| !l.contains(start_line))
    .skip(1)
    .take_while(|(_, l)| !l.contains(end_line))
    .map(|(i, l)| parser(l, Some(i)))
    .collect()
}
