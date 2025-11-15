use std::{collections::BTreeMap, io::ErrorKind};

use crate::{
    memtable::Memtable,
    ss_table::{
        SsTableError,
        ss_table_impl::{SsTable, SsTableName},
    },
    types::{Key, Value},
};

/// Manages the Sorted Strings Tables.
pub(crate) struct SsTableManager {
    ss_tables: BTreeMap<SsTableName, SsTable>,
}

impl SsTableManager {
    const SS_TABLES_DIR: &str = "./ss_tables";

    /// Creates a new [`SsTableManager`].
    fn new() -> Result<Self, SsTableError> {
        // Check the existence of the `ss_tables` directory.
        match std::fs::create_dir(Self::SS_TABLES_DIR) {
            Ok(_) => (),
            Err(err) if err.kind() == ErrorKind::AlreadyExists => (),
            Err(err) => return Err(SsTableError::Io(err.to_string())),
        }

        let mut ss_tables = BTreeMap::new();
        for ss_table_file in std::fs::read_dir(Self::SS_TABLES_DIR).expect(&format!(
            "Directory: `{}` should exist",
            Self::SS_TABLES_DIR
        )) {
            let dir_entry = ss_table_file.map_err(|e| SsTableError::Io(e.to_string()))?;
            let ss_table_name = SsTableName(dir_entry.file_name().into_string().unwrap());
            let ss_table = SsTable::new(ss_table_name.clone(), dir_entry.path().into_boxed_path());
            ss_tables.insert(ss_table_name, ss_table);
        }
        Ok(Self { ss_tables })
    }

    /// Flushes a [`Memtable`] to disk.
    fn flush_memtable(memtable: &dyn Memtable) -> Result<(), SsTableError> {
        todo!()
    }

    /// Retrieves the value from the SS table associated with the given key.
    fn get(&self, key: &Key) -> Result<Value, SsTableError> {
        todo!()
    }
}
