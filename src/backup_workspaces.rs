use serde::Deserialize;
use serde::Serialize;

use crate::empty_window::EmptyWindow;
use crate::folder::Folder;
use crate::workspace::Workspace;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWorkspaces {
    pub workspaces: Vec<Workspace>,
    pub folders: Vec<Folder>,
    pub empty_windows: Vec<EmptyWindow>,
}
