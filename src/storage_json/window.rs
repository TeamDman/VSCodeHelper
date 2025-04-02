use super::ui_state::UiState;
use super::workspace_identifier::WorkspaceIdentifier;
use crate::storage_json::uri::Uri;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

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
