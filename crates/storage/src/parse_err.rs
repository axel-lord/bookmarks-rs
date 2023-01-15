use thiserror::Error;

#[derive(Clone, Debug, Error)]
/// Errors representing failure to parse some content.
pub enum ParseErr {
    /// If some line was unsuccessfully parsed optionally has which line and/or a
    /// message.
    #[error(
        "could not parse line {}: {}",
        .1.map(|v| v.to_string()).unwrap_or_else(|| String::from("<unknown>")),
        .0.clone().unwrap_or_else(|| String::from("<unknown>")),
    )]
    Line(Option<String>, Option<usize>),
    /// Some other issue parsing with a message.
    #[error("parse issue: {0}")]
    Other(String),
}

impl From<std::io::Error> for ParseErr {
    fn from(err: std::io::Error) -> Self {
        ParseErr::Other(format!("{err}"))
    }
}
