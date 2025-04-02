use std::path::PathBuf;

use super::keys::Key;
use super::models::Item;
use crate::state_vscdb::schema::ItemTable::dsl::*;
use crate::storage_json::paths::VSCodePath;
use diesel::SqliteConnection;
use diesel::prelude::*;

pub struct StateVscdb {
    pub connection: SqliteConnection,
}
impl StateVscdb {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }
    pub fn try_default() -> eyre::Result<Self> {
        let vscdb_path: PathBuf = VSCodePath::StateVscdb.try_into()?;
        let connection = SqliteConnection::establish(&vscdb_path.to_string_lossy())?;
        Ok(Self { connection })
    }
}
impl StateVscdb {
    pub fn read<K: Key>(&mut self) -> eyre::Result<K::Value> {
        let result = ItemTable
            .filter(key.eq(K::KEY))
            .select(Item::as_select())
            .first::<Item>(&mut self.connection)?;
        let rtn: K::Value = serde_json::from_slice(&result.value)?;
        Ok(rtn)
    }
}
