#![deny(missing_docs)]
//! Create an in-memory key/value store

pub use error::{KvsError, Result};
pub use kv::KvStore;

mod error;
mod kv;
