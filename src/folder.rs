use serde::Deserialize;
use serde::Serialize;

use crate::uri::Uri;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub folder_uri: Uri,
}
