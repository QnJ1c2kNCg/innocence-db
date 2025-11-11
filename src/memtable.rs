pub(crate) mod btree_map;

use crate::{
    db::DbError,
    types::{Key, Value},
};

/// TODO
pub(crate) trait MemTable {
    /// Inserts a key-value pair into the memtable.
    fn insert(&self, key: Key, value: Value) -> Result<(), MemTableError>;

    /// Retrieves the value from the memtable associated with the given key.
    fn get(&self, key: &Key) -> Result<Value, MemTableError>;
}

/// Represents errors that can occur when interacting with a memtable.
pub(crate) enum MemTableError {
    /// The requested key was not found in the memtable.
    KeyNotFound,
}

impl From<MemTableError> for DbError {
    fn from(error: MemTableError) -> Self {
        match error {
            MemTableError::KeyNotFound => DbError::KeyNotFound,
        }
    }
}
