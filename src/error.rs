use failure::Fail;
use std::io;

/// Error type for kvs
#[derive(Debug, Fail)]
pub enum KvsError {
    /// input/utput error
    #[fail(display = "io error")]
    Io(#[cause] io::Error),
    /// key not found error
    #[fail(display = "Key not found")]
    KeyNotFound,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
