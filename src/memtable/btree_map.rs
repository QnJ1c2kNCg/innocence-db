use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    memtable::{Memtable, MemtableError},
    types::{Key, Value},
};

#[derive(Default)]
pub(crate) struct BTreeMapMemtable {
    // TODO: RefCell for now, but probably locking in the future.
    map: RefCell<BTreeMap<Key, Value>>,
}

impl Memtable for BTreeMapMemtable {
    fn insert(&self, key: Key, value: Value) -> Result<(), MemtableError> {
        self.map.borrow_mut().insert(key.clone(), value.clone());
        Ok(())
    }

    fn get(&self, key: &Key) -> Result<Value, MemtableError> {
        self.map
            .borrow()
            .get(key)
            .cloned()
            .ok_or(MemtableError::KeyNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert_get() {
        let memtable = BTreeMapMemtable::default();

        for i in 0i32..10 {
            assert!(matches!(
                memtable.get(&i.to_be_bytes().into()).unwrap_err(),
                MemtableError::KeyNotFound
            ));
            memtable
                .insert(i.to_be_bytes().into(), i.to_be_bytes().into())
                .unwrap();
            assert_eq!(
                memtable.get(&i.to_be_bytes().into()).unwrap(),
                &i.to_be_bytes()
            );
        }
    }
}
