use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::empty_window::EmptyWindow;
use crate::storage_json::folder::Folder;
use crate::storage_json::workspace::WorkspaceIdentifier;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWorkspaces {
    pub workspaces: Vec<WorkspaceIdentifier>,
    pub folders: Vec<Folder>,
    pub empty_windows: Vec<EmptyWindow>,
}
