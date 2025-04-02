use crate::storage_json::backup_folder_id::BackupFolderId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyWindow {
    pub backup_folder: BackupFolderId,
}
