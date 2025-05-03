use std::fmt;

use super::database;

#[derive(Debug)]
pub enum Error {
  DatabaseInitialization(database::error::Error),
  DatabaseLoad(database::error::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::DatabaseInitialization(e) => write!(f, "[database initializaton]: {}", e.to_string()),
      Error::DatabaseLoad(e) => write!(f, "[database deserilization]: {}", e.to_string()),
    }
  }
}
