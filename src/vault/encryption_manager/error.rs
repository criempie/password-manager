use std::fmt;

#[derive(Debug)]
pub enum Error {
    ParamsInvalidLength,
    PaddingOutputBufferInvalid,
    UnPaddingError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParamsInvalidLength => {
                write!(f, "Encryption error: Data in params has invalid length")
            }
            Error::PaddingOutputBufferInvalid => write!(
                f,
                "Encryption error: Output buffer for padding has invalid length"
            ),
            Error::UnPaddingError => write!(f, "Encryption error: UnPadding"),
        }
    }
}
