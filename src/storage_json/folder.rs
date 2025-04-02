use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::uri::Uri;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub folder_uri: Uri,
}
