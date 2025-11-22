use crate::storage_json::uri::Uri;
use crate::storage_json::workspace_id::WorkspaceId;
use crate::workspace_json::HasWorkspacePath;
use serde::Deserialize;
use serde::Serialize;

use super::Key;

pub struct HistoryRecentlyOpenedPathsListKey;
impl Key for HistoryRecentlyOpenedPathsListKey {
    const KEY: &'static str = "history.recentlyOpenedPathsList";
    type Value = HistoryRecentlyOpenedPathsListValue;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryRecentlyOpenedPathsListValue {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    #[serde(rename_all = "camelCase")]
    Folder { folder_uri: Uri },
    #[serde(rename_all = "camelCase")]
    File { file_uri: Uri },
    #[serde(rename_all = "camelCase")]
    Workspace { workspace: Workspace },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: WorkspaceId,
    pub config_path: Uri,
}

impl HasWorkspacePath for Workspace {
    fn workspace_path(&self) -> Uri {
        self.config_path.clone()
    }
}

#[cfg(test)]
mod test {
    use super::HistoryRecentlyOpenedPathsListKey;
    use crate::state_vscdb::VSCodeStateVscdb;

    #[test]
    fn it_works() -> eyre::Result<()> {
        let mut state_vscdb = VSCodeStateVscdb::try_default()?;
        let recently_opened = state_vscdb.read::<HistoryRecentlyOpenedPathsListKey>()?;
        println!("Found {} entries", recently_opened.entries.len());
        Ok(())
    }
}
