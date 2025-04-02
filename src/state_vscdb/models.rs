use diesel::prelude::*;
use serde::Deserialize;

use super::keys::Key;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::state_vscdb::schema::ItemTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub key: String,
    pub value: Vec<u8>,
}
impl Item {
    pub fn parse<K: Key>(&self) -> eyre::Result<K::Value> {
        if self.key != K::KEY {
            return Err(eyre::eyre!("Key mismatch: expected {}, found {}", K::KEY, self.key));
        }
        let value = serde_json::from_slice(&self.value)?;
        Ok(value)
    }
    pub fn value_as_string(&self) -> eyre::Result<String> {
        let value = String::from_utf8(self.value.clone())?;
        Ok(value)
    }
}

#[derive(Deserialize, Debug)]
pub struct RecentlyOpenedPathsList {
    pub entries: Vec<RecentlyOpenedEntry>,
}

#[derive(Deserialize, Debug)]
pub struct RecentlyOpenedEntry {
    #[serde(rename = "fileUri")]
    pub file_uri: Option<String>,
    #[serde(rename = "folderUri")]
    pub folder_uri: Option<String>,
    pub label: Option<String>,
}
