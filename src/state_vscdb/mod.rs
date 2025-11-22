pub mod keys;
pub mod models;
pub mod schema;

use crate::state_vscdb::schema::ItemTable::dsl::*;
use crate::storage_json::paths::VSCodePath;
use diesel::SqliteConnection;
use diesel::prelude::*;
use keys::Key;
use models::Item;
use std::path::PathBuf;

pub struct VSCodeStateVscdb {
    pub connection: SqliteConnection,
}
impl VSCodeStateVscdb {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }
    pub fn try_default() -> eyre::Result<Self> {
        let vscdb_path: PathBuf = VSCodePath::StateVscdb.try_into()?;
        let connection = SqliteConnection::establish(&vscdb_path.to_string_lossy())?;
        Ok(Self { connection })
    }
    pub fn keys(&mut self) -> eyre::Result<Vec<String>> {
        let keys = ItemTable
            .select(key)
            .distinct()
            .load::<String>(&mut self.connection)?;
        Ok(keys)
    }
    pub fn entries(&mut self) -> eyre::Result<Vec<Item>> {
        let entries = ItemTable
            .select(Item::as_select())
            .load::<Item>(&mut self.connection)?;
        Ok(entries)
    }
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
