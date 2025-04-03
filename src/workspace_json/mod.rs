use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::storage_json::uri::Uri;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceJson {
    pub folders: Vec<PathHolder>,
    pub settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathHolder {
    pub path: PathBuf,
}

pub trait HasWorkspacePath {
    fn workspace_path(&self) -> Uri;

    fn read(&self) -> eyre::Result<WorkspaceJson> {
        let config_path = self.workspace_path().as_path()?;
        let workspace_json: WorkspaceJson =
            serde_json::from_reader(std::fs::File::open(config_path)?)?;
        Ok(workspace_json)
    }
}
