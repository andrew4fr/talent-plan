#![deny(missing_docs)]
//! Create an in-memory key/value store

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
pub struct KvStore {}

impl KvStore {
    /// Creates a strore
    pub fn new() -> Self {
        KvStore {}
    }

    /// Sets the value of a key
    pub fn set(&self, key: String, value: String) {
        unimplemented!()
    }

    /// Gets the value of a key
    ///
    /// Returns None if a key doesn't exist
    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }

    /// Removes a key from store
    pub fn remove(&self, key: String) {
        unimplemented!()
    }
}
