#![deny(missing_docs)]
//! Create an in-memory key/value store

use std::path::PathBuf;
use failure::Fail;
use std::io;

/// The `KvStore` stores key/value pairs
/// The pairs are stored in memory
/// Example:
/// ```rust
/// # use kvs::KvStore
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let value = store.get("key".to_owned());
/// assert_eq!(value, Some("value".to_owned());
/// ```
pub struct KvStore {
    path: PathBuf
}

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

impl KvStore {
    /// Sets the value of a key
    pub fn set(&self, key: String, value: String) -> Result<()> {
        Ok(())
    }

    /// Gets the value of a key
    ///
    /// Returns None if a key doesn't exist
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(None)
    }

    /// Removes a key from store
    pub fn remove(&self, key: String) -> Result<()> {
        Err(KvsError::KeyNotFound)
    }

    /// Creates new store in tmp directory
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        Ok(KvStore {path})
    }
}
