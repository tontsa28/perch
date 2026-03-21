use std::{
    error::Error as StdError, fmt::Display, num::ParseIntError, result::Result as StdResult,
};

pub(crate) type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub(crate) struct Error {
    kind: ErrorKind,
    source: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    // Creates a new instance of `Error`.
    pub(crate) fn new<E>(kind: ErrorKind, source: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Error {
            kind,
            source: source.into(),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.source.as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.source)
    }
}

impl From<&str> for Error {
    fn from(source: &str) -> Self {
        Error::new(ErrorKind::InvalidFen, source)
    }
}

impl From<ParseIntError> for Error {
    fn from(source: ParseIntError) -> Self {
        Error::new(ErrorKind::ParseInt, source)
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ErrorKind {
    InvalidFen,
    ParseInt,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFen => write!(f, "Invalid FEN"),
            Self::ParseInt => write!(f, "Failed to parse integer"),
        }
    }
}
