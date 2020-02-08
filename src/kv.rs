use crate::{KvsError, Result};
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::Deserializer;

/// The `KvStore` stores key/value pairs
/// The pairs are stored in memory
///
/// Example:
/// ```rust
/// # use kvs::KvStore
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let value = store.get("key".to_owned());
/// assert_eq!(value, Some("value".to_owned());
/// ```
pub struct KvStore {
    path: PathBuf,
}

impl KvStore {
    /// Creates new store in tmp directory
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        Ok(KvStore { path })
    }

    /// Sets the value of a key
    pub fn set(&self, key: String, value: String) -> Result<()> {
        let cmd = Command::set(key, value);
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
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Rm { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Self {
        Command::Set{key, value}
    }

    fn rm(key: String) -> Self {
        Command::Rm{key}
    }
}
