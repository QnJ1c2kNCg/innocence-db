use std::collections::BTreeMap;

use crate::{
    memtable::{MemTable, MemTableError},
    types::{Key, Value},
};

struct BTreeMapMemTable {
    map: BTreeMap<Key, Value>,
}

impl MemTable for BTreeMapMemTable {
    fn insert(&mut self, key: Key, value: Value) -> Result<(), MemTableError> {
        self.map.insert(key.clone(), value.clone());
        Ok(())
    }
}
