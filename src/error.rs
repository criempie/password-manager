use std::fmt;

use crate::vault::entry;

#[derive(Debug)]
pub enum ApplicationError {
    EntryError(entry::Error),
}

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
    EncryptionKeyInvalid,
}

#[derive(Debug)]
pub enum CryptError {
    Any(String),
    KeyInvalidLength,
    IvInvalidLength,
    FoldInvalidLength,
    Base64DecodeError(String),
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
            VaultError::EncryptionKeyInvalid => write!(f, "Vault error: Encryption key invalid."),
            VaultError::Any(message) => write!(f, "Vault error: {}.", message),
        }
    }
}

impl fmt::Display for CryptError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CryptError::Any(message) => write!(f, "Crypt error: {}.", message),
            CryptError::IvInvalidLength => write!(f, "Crypt error: iv invalid length."),
            CryptError::KeyInvalidLength => write!(f, "Crypt error: key invalid length."),
            CryptError::Base64DecodeError(message) => {
                write!(f, "Crypt error: base64 decode failed: {}.", message)
            }
            CryptError::FoldInvalidLength => {
                write!(f, "Crypt error: fold of key and iv has invalid length.")
            }
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
impl std::error::Error for CryptError {}
