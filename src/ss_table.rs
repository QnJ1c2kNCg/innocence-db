pub(crate) mod ss_table_impl;
pub(crate) mod ss_table_manager;

pub(crate) enum SsTableError {
    Io(String),
}
