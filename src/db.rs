use crate::{
    memtable::{Memtable, btree_map::BTreeMapMemtable},
    types::{Key, Value},
};

/// The innocence database.
pub struct Db {
    memtable: Box<dyn Memtable>,
}

/// All publicly exposed errors.
#[derive(Debug)]
pub enum DbError {
    /// The requested key was not found in the database.
    KeyNotFound,
    /// Errors associated with the database builder.
    DbBuilder(String),
}

impl Db {
    /// Inserts a key-value pair into the database.
    pub fn insert(&self, key: Key, value: Value) -> Result<(), DbError> {
        self.memtable.insert(key, value).map_err(|_| todo!())
    }

    /// Retrieves the value from the database associated with the given key.
    // TODO: Maybe return a reference to the value.
    pub fn get(&self, key: &Key) -> Result<Value, DbError> {
        self.memtable.get(key).map_err(|e| e.into())
    }
}

/// Builder for the [`Db`].
pub struct DbBuilder {
    memtable: Option<Box<dyn Memtable>>,
}

impl DbBuilder {
    /// Creates a new database builder.
    pub fn new() -> Self {
        Self { memtable: None }
    }

    /// Sets the memtable for the database.
    pub fn memtable(mut self, memtable: Box<dyn Memtable>) -> Self {
        self.memtable = Some(memtable);
        self
    }

    /// Builds the database.
    pub fn build(self) -> Result<Db, DbError> {
        let memtable = self
            .memtable
            .ok_or(DbError::DbBuilder("Memtable not provided".to_string()))?;
        Ok(Db { memtable })
    }

    /// Builds the database for testing purposes.
    // TODO: Should probably clean this up.
    pub fn build_for_tests(self) -> Result<Db, DbError> {
        let memtable = Box::new(BTreeMapMemtable::default());
        self.memtable(memtable).build()
    }
}
