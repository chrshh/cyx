use core::fmt;
use std::{error::Error, io};

#[derive(Debug)]
pub enum CGrepError {
    PatternMissing,
    PathMissing,
    UnknownFlag(String),
    NotFound(String),
    IsDir(String),
    IOError(io::Error),
    PoolInitError,
}

impl fmt::Display for CGrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CGrepError::IOError(e) => write!(f, "cgrep: IO error: {}", e),
            CGrepError::PatternMissing => write!(f, "cgrep: missing search pattern"),
            CGrepError::PathMissing => write!(f, "cgrepL missing search path"),
            CGrepError::UnknownFlag(flag) => write!(f, "cgrep: unknown flag: -{}", flag),
            CGrepError::NotFound(haystack) => write!(f, "cgrep: file does not exist: {}", haystack),
            CGrepError::IsDir(haystack) => write!(f, "cgrep: {} is a directory", haystack),
            CGrepError::PoolInitError => write!(f, "cgrep: failed to create thread pool"),
        }
    }
}

impl Error for CGrepError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CGrepError::IOError(e) => Some(e),
            _ => None,
        }
    }
}

/* Enable ? operator conversion */
impl From<io::Error> for CGrepError {
    fn from(err: io::Error) -> Self {
        CGrepError::IOError(err)
    }
}
