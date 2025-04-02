use super::uri::Uri;
use super::workspace_id::WorkspaceId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceIdentifier {
    pub id: WorkspaceId,
    #[serde(rename = "configURIPath")]
    pub config_uri_path: Uri,
}
