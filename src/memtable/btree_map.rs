use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    memtable::{MemTable, MemTableError},
    types::{Key, Value},
};

#[derive(Default)]
pub(crate) struct BTreeMapMemTable {
    // TODO: RefCell for now, but probably locking in the future.
    map: RefCell<BTreeMap<Key, Value>>,
}

impl MemTable for BTreeMapMemTable {
    fn insert(&self, key: Key, value: Value) -> Result<(), MemTableError> {
        self.map.borrow_mut().insert(key.clone(), value.clone());
        Ok(())
    }

    fn get(&self, key: &Key) -> Result<Value, MemTableError> {
        self.map
            .borrow()
            .get(key)
            .cloned()
            .ok_or(MemTableError::KeyNotFound)
    }
}
