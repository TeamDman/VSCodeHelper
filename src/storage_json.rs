use std::path::PathBuf;

use crate::machine_id::MachineId;
use crate::paths::VSCodePath;
use serde::Deserialize;
use serde::Serialize;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct VSCodeStorageJson {
    #[serde(rename = "telemetry.machineId")]
    pub telemetry_machine_id: MachineId,
    pub theme: Theme,
}

impl VSCodeStorageJson {
    pub fn load_from_disk() -> eyre::Result<Self> {
        let json_path: PathBuf = VSCodePath::StorageJson.try_into()?;
        debug!("Trying to load storage json from: {}", json_path.display());
        let json = std::fs::read_to_string(json_path)?;
        let storage_json: Self = serde_json::from_str(&json)?;
        Ok(storage_json)
    }
}
