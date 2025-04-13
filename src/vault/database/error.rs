use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnhandledIO(String),
    Serialization,
    Deserialization,
    WhileOpeningFile(String),
    WhileWritingIntoFile,
    WhileReadingFile,
    WhileCreatingFile,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Serialization => write!(f, "Database error: Serialization"),
            Error::Deserialization => write!(f, "Database error: Deserialization"),
            Error::UnhandledIO(message) => write!(f, "Database error: unhandled io: {}", message),
            Error::WhileOpeningFile(file_path) => {
                write!(f, "Database error: cannot open file {}", file_path)
            }
            Error::WhileWritingIntoFile => {
                write!(f, "Database error: file while try to write into the file")
            }
            Error::WhileReadingFile => write!(f, "Database error: fail while try to read the file"),
            Error::WhileCreatingFile => {
                write!(f, "Database error: fail while try to create the file")
            }
        }
    }
}
