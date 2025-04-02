use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::uri::Uri;
use crate::storage_json::workspace_id::WorkspaceId;
use crate::workspace_json::WorkspaceJson;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceIdentifier {
    pub id: WorkspaceId,
    #[serde(rename = "configURIPath")]
    pub config_uri_path: Uri,
}
impl WorkspaceIdentifier {
    pub fn read(&self) -> eyre::Result<WorkspaceJson> {
        let config_path = self.config_uri_path.as_path()?;
        let workspace_json: WorkspaceJson =
            serde_json::from_reader(std::fs::File::open(config_path)?)?;
        Ok(workspace_json)
    }
}
