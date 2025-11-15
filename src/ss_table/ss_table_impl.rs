use std::path::Path;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct SsTableName(pub(crate) String);

pub(crate) struct SsTable {
    name: SsTableName,
    path: Box<Path>,
}

impl SsTable {
    pub(crate) fn new(name: SsTableName, path: Box<Path>) -> Self {
        Self { name, path }
    }
}
