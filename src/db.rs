use crate::{
    memtable::MemTable,
    types::{Key, Value},
};

/// The innocence database.
struct Db {
    memtable: Box<dyn MemTable>,
}

/// All publicly exposed errors.
enum DbError {
    /// The requested key was not found in the database.
    KeyNotFound,
    /// Errors associated with the database builder.
    DbBuilder(String),
}

impl Db {
    /// Inserts a key-value pair into the database.
    pub(crate) fn insert(&self, key: Key, value: Value) -> Result<(), DbError> {
        todo!()
    }

    /// Retrieves the value from the database associated with the given key.
    pub(crate) fn get(&self, key: &Key) -> Result<&Value, DbError> {
        todo!()
    }
}

/// Builder for the [`Db`].
struct DbBuilder {
    memtable: Option<Box<dyn MemTable>>,
}

impl DbBuilder {
    /// Creates a new database builder.
    pub fn new() -> Self {
        Self { memtable: None }
    }

    /// Builds the database.
    pub fn build(self) -> Result<Db, DbError> {
        let memtable = self
            .memtable
            .ok_or(DbError::DbBuilder("Memtable not provided".to_string()))?;
        Ok(Db { memtable })
    }
}
