use diesel::prelude::*;
use serde::Deserialize;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::state_vscdb::schema::ItemTable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub key: String,
    pub value: Vec<u8>,
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
