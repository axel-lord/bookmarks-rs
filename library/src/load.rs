use std::{error::Error, fs::File, io};

#[derive(Clone, Debug)]
pub enum ParseErr {
    Line(Option<String>, Option<usize>),
    Other(String),
}

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErr::Line(Some(l), None) => {
                write!(f, "could not parse line: {l}")
            }
            ParseErr::Line(Some(l), Some(i)) => {
                write!(f, "could not parse line {i}: {l}")
            }
            ParseErr::Line(None, None) => {
                write!(f, "could not parse anything")
            }
            ParseErr::Line(None, Some(i)) => {
                write!(f, "could not parse line {i}")
            }
            ParseErr::Other(s) => write!(f, "{s}"),
        }
    }
}

impl Error for ParseErr {}

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
