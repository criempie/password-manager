use std::fmt;

#[derive(Debug)]
pub enum Error {
  Unhandled(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Unhandled(message) => write!(f, "Vault error: unhandled: {}", message),
    }
  }
}
