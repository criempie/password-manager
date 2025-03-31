use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    Any(String),
    FileNotFound(),
    FileAlreadyExist(),
    FileFormatInvalid(),
}

#[derive(Debug)]
pub enum VaultError {
    Any(String),
    EntryNotFound,
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::FileNotFound() => write!(f, "Database error: File not found."),
            DatabaseError::FileAlreadyExist() => write!(f, "Database error: File already exist."),
            DatabaseError::FileFormatInvalid() => write!(f, "Database error: File format invalid."),
            DatabaseError::Any(message) => write!(f, "Database error: {}.", message),
        }
    }
}

impl fmt::Display for VaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VaultError::EntryNotFound => write!(f, "Vault error: Entry not found."),
            VaultError::Any(message) => write!(f, "Vault error: {}.", message),
        }
    }
}

// impl From<io::Error> for Error {
//     fn from(error: io::Error) -> Self {
//         Error::DatabaseError(error.to_string())
//     }
// }

impl std::error::Error for DatabaseError {}
impl std::error::Error for VaultError {}
