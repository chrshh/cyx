use core::fmt;
use std::{error::Error, io};

#[derive(Debug)]
pub enum CFindError {
    PatternMissing,
    UnknownFlag(String),
    MissingFlag,
    IOError(io::Error),
    PoolInitError,
    UnkownEntryType(String),
    NotFound(String),
    IsFile(String),
}

impl fmt::Display for CFindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CFindError::PatternMissing => write!(f, "cfind: missing search pattern"),
            CFindError::UnknownFlag(flag) => write!(f, "cfind: unknown flag: -{}", flag),
            CFindError::MissingFlag => write!(f, "cfind: flag symbol found with no flag"),
            CFindError::IOError(e) => write!(f, "cfind: IO error: {}", e),
            CFindError::PoolInitError => write!(f, "cfind: failed to create thread pool"),
            CFindError::UnkownEntryType(t) => write!(f, "cfind: unknown entry type: {}", t),
            CFindError::NotFound(s) => write!(f, "cfind: path not found: {}", s),
            CFindError::IsFile(file) => write!(f, "cfind: provided path is a file: {}", file),
        }
    }
}

impl Error for CFindError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CFindError::IOError(e) => Some(e),
            _ => None,
        }
    }
}

/* Enable ? operator conversion */
impl From<io::Error> for CFindError {
    fn from(err: io::Error) -> Self {
        CFindError::IOError(err)
    }
}
