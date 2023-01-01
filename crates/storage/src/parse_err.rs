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

// impl std::fmt::Display for ParseErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ParseErr::Line(Some(l), None) => {
//                 write!(f, "could not parse line: {l}")
//             }
//             ParseErr::Line(Some(l), Some(i)) => {
//                 write!(f, "could not parse line {i}: {l}")
//             }
//             ParseErr::Line(None, None) => {
//                 write!(f, "could not parse anything")
//             }
//             ParseErr::Line(None, Some(i)) => {
//                 write!(f, "could not parse line {i}")
//             }
//             ParseErr::Other(s) => write!(f, "{s}"),
//         }
//     }
// }
//
// impl std::error::Error for ParseErr {}

impl From<std::io::Error> for ParseErr {
    fn from(err: std::io::Error) -> Self {
        ParseErr::Other(format!("{err}"))
    }
}
