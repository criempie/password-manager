use serde_json;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DatabaseError(message) => write!(f, "Database error: {}.", message),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::DatabaseError(error.to_string())
    }
}

impl std::error::Error for Error {}
