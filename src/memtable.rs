pub(crate) mod btree_map;

use crate::{
    db::DbError,
    types::{Key, Value},
};

/// TODO
pub(crate) trait Memtable {
    /// Inserts a key-value pair into the memtable.
    fn insert(&self, key: Key, value: Value) -> Result<(), MemtableError>;

    /// Retrieves the value from the memtable associated with the given key.
    fn get(&self, key: &Key) -> Result<Value, MemtableError>;
}

/// Represents errors that can occur when interacting with a memtable.
#[derive(Debug)]
pub(crate) enum MemtableError {
    /// The requested key was not found in the memtable.
    KeyNotFound,
}

impl From<MemtableError> for DbError {
    fn from(error: MemtableError) -> Self {
        match error {
            MemtableError::KeyNotFound => DbError::KeyNotFound,
        }
    }
}
