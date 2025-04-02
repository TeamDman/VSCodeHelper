use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use crate::backup_folder_id::BackupFolderId;
use crate::profile_dunder::ProfileDunder;
use crate::uri::Uri;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAssociations {
    pub workspaces: HashMap<Uri, ProfileDunder>,
    pub empty_windows: HashMap<BackupFolderId, ProfileDunder>,
}
