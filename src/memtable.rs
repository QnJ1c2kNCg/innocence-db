pub(crate) mod btree_map;

use crate::types::{Key, Value};

/// TODO
pub(crate) trait MemTable {
    /// Inserts a key-value pair into the memtable.
    fn insert(&mut self, key: Key, value: Value) -> Result<(), MemTableError>;
}

pub(crate) enum MemTableError {}
