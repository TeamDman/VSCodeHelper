use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::uri::Uri;
use crate::storage_json::workspace_id::WorkspaceId;
use crate::workspace_json::HasWorkspacePath;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceIdentifier {
    pub id: WorkspaceId,
    #[serde(rename = "configURIPath")]
    pub config_uri_path: Uri,
}

impl HasWorkspacePath for WorkspaceIdentifier {
    fn workspace_path(&self) -> Uri {
        self.config_uri_path.clone()
    }
}
