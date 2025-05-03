use std::fmt;

#[derive(Debug)]
pub enum Error {
  Unhandled(String),
  DatabaseFileNotFound,
  FailWhileWritingToFile,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Unhandled(message) => write!(f, "Vault error: unhandled: {}", message),
      Error::DatabaseFileNotFound => write!(f, "file not found"),
      Error::FailWhileWritingToFile => write!(f, "fail while writing to file"),
    }
  }
}
