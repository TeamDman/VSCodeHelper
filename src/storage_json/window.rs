use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use crate::storage_json::uri::Uri;
use super::ui_state::UiState;
use super::workspace_identifier::WorkspaceIdentifier;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Window {
    #[serde(rename_all = "camelCase")]
    FolderWindow {
        folder: Uri,
        backup_path: Option<PathBuf>,
        ui_state: UiState,
    },
    #[serde(rename_all = "camelCase")]
    WorkspaceWindow {
        workspace_identifier: WorkspaceIdentifier,
        backup_path: Option<PathBuf>,
        ui_state: UiState,
    },
}
