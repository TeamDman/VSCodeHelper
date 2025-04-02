use serde::Deserialize;
use serde::Serialize;

use crate::uri::Uri;
use crate::workspace_id::WorkspaceId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: WorkspaceId,
    #[serde(rename="configURIPath")]
    pub config_uri_path: Uri,
}