use super::ui_state::UiState;
use super::workspace::WorkspaceIdentifier;
use crate::storage_json::uri::Uri;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Window {
    FolderWindow {
        folder: Uri,
        #[serde(rename = "backupPath")]
        backup_path: Option<PathBuf>,
        #[serde(rename = "uiState")]
        ui_state: UiState,
    },
    WorkspaceWindow {
        #[serde(rename = "workspaceIdentifier")]
        workspace_identifier: WorkspaceIdentifier,
        #[serde(rename = "backupPath")]
        backup_path: Option<PathBuf>,
        #[serde(rename = "uiState")]
        ui_state: UiState,
    },
}

impl<'de> Deserialize<'de> for Window {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Helper structs for each variant
        #[derive(Deserialize)]
        struct WorkspaceWindow {
            #[serde(rename = "workspaceIdentifier")]
            workspace_identifier: WorkspaceIdentifier,
            #[serde(rename = "backupPath")]
            backup_path: Option<PathBuf>,
            #[serde(rename = "uiState")]
            ui_state: UiState,
        }

        #[derive(Deserialize)]
        struct FolderWindow {
            folder: Uri,
            #[serde(rename = "backupPath")]
            backup_path: Option<PathBuf>,
            #[serde(rename = "uiState")]
            ui_state: UiState,
        }

        // First deserialize to a generic Value to inspect fields
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("workspaceIdentifier").is_some() {
            if value.get("folder").is_some() {
                return Err(serde::de::Error::custom(
                    "Window cannot have both 'workspaceIdentifier' and 'folder' fields",
                ));
            }

            let window: WorkspaceWindow = serde_json::from_value(value)
                .map_err(|e| serde::de::Error::custom(format!("Invalid WorkspaceWindow: {}", e)))?;

            Ok(Window::WorkspaceWindow {
                workspace_identifier: window.workspace_identifier,
                backup_path: window.backup_path,
                ui_state: window.ui_state,
            })
        } else if value.get("folder").is_some() {
            let window: FolderWindow = serde_json::from_value(value)
                .map_err(|e| serde::de::Error::custom(format!("Invalid FolderWindow: {}", e)))?;

            Ok(Window::FolderWindow {
                folder: window.folder,
                backup_path: window.backup_path,
                ui_state: window.ui_state,
            })
        } else {
            Err(serde::de::Error::custom(
                "Window must contain either 'workspaceIdentifier' or 'folder' field",
            ))
        }
    }
}
