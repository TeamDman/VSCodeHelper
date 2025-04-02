use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::uri::Uri;
use crate::storage_json::workspace_id::WorkspaceId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: WorkspaceId,
    #[serde(rename = "configURIPath")]
    pub config_uri_path: Uri,
}
