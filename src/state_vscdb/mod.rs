pub mod keys;
pub mod models;
pub mod schema;

use crate::state_vscdb::schema::ItemTable::dsl::{ItemTable, key};
use crate::storage_json::paths::VSCodePath;
use diesel::prelude::*;
use keys::Key;
use models::Item;
use std::path::PathBuf;

#[expect(
    missing_debug_implementations,
    reason = "diesel::sqlite::SqliteConnection does not implement Debug"
)]
pub struct VSCodeStateVscdb {
    pub connection: diesel::sqlite::SqliteConnection,
}
impl VSCodeStateVscdb {
    #[must_use]
    pub fn new(connection: diesel::sqlite::SqliteConnection) -> Self {
        Self { connection }
    }

    /// Opens the default VS Code `state.vscdb` database.
    ///
    /// # Errors
    /// Returns an error if the database path cannot be resolved or the `SQLite` connection fails.
    pub fn try_default() -> eyre::Result<Self> {
        let vscdb_path: PathBuf = VSCodePath::StateVscdb.try_into()?;
        let connection = SqliteConnection::establish(&vscdb_path.to_string_lossy())?;
        Ok(Self { connection })
    }

    /// Returns all distinct keys stored in the state database.
    ///
    /// # Errors
    /// Returns an error if the database query fails.
    pub fn keys(&mut self) -> eyre::Result<Vec<String>> {
        let keys = ItemTable
            .select(key)
            .distinct()
            .load::<String>(&mut self.connection)?;
        Ok(keys)
    }

    /// Returns all entries stored in the state database.
    ///
    /// # Errors
    /// Returns an error if the database query fails.
    pub fn entries(&mut self) -> eyre::Result<Vec<Item>> {
        let entries = ItemTable
            .select(Item::as_select())
            .load::<Item>(&mut self.connection)?;
        Ok(entries)
    }

    /// Reads and parses a well-known key value from the state database.
    ///
    /// # Errors
    /// Returns an error if the query fails, no row is found, or value parsing fails.
    pub fn read<K: Key>(&mut self) -> eyre::Result<K::Value> {
        let item = ItemTable
            .filter(key.eq(K::KEY))
            .select(Item::as_select())
            .first::<Item>(&mut self.connection)?;
        let rtn = item.parse::<K>()?;
        Ok(rtn)
    }
}

pub mod well_known_keys {
    pub use super::keys::history_recently_opened_paths_list::HistoryRecentlyOpenedPathsListKey;
}
