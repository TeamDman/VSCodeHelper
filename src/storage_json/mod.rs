pub mod backup_folder_id;
pub mod backup_workspaces;
pub mod color;
pub mod color_info;
pub mod empty_window;
pub mod folder;
pub mod layout_info;
pub mod machine_id;
pub mod paths;
pub mod profile_associations;
pub mod profile_dunder;
pub mod side;
pub mod telemetry_dev_device_id;
pub mod telemetry_sqm_id;
pub mod theme;
pub mod ui_state;
pub mod uri;
pub mod window;
pub mod window_splash;
pub mod windows_state;
pub mod workspace;
pub mod workspace_id;

use crate::storage_json::backup_workspaces::BackupWorkspaces;
use crate::storage_json::color::Color;
use crate::storage_json::machine_id::MachineId;
use crate::storage_json::paths::VSCodePath;
use crate::storage_json::profile_associations::ProfileAssociations;
use crate::storage_json::telemetry_dev_device_id::TelemetryDevDeviceId;
use crate::storage_json::telemetry_sqm_id::TelemetrySqmId;
use crate::storage_json::theme::Theme;
use crate::storage_json::window_splash::WindowSplash;
use crate::storage_json::windows_state::WindowsState;
use eyre::Context;
use eyre::eyre;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VSCodeStorageJson {
    #[serde(rename = "telemetry.machineId")]
    pub telemetry_machine_id: MachineId,
    pub theme: Theme,
    pub theme_background: Color,
    pub windows_state: WindowsState,
    pub picker_working_dir: PathBuf,
    #[serde(rename = "quit.from.restart")]
    pub quit_from_restart: bool,
    pub window_splash: WindowSplash,
    pub window_control_height: u32,
    pub backup_workspaces: BackupWorkspaces,
    pub user_data_profiles_migration: bool,
    pub profile_associations: ProfileAssociations,
    pub profile_associations_migration: bool,
    #[serde(rename = "window.experimental.useSandbox")]
    pub window_experimental_use_sandbox: bool,
    #[serde(rename = "telemetry.sqmId")]
    pub telemetry_sqm_id: TelemetrySqmId,
    #[serde(rename = "telemetry.devDeviceId")]
    pub telemetry_dev_device_id: TelemetryDevDeviceId,
}

impl VSCodeStorageJson {
    pub fn load_from_disk() -> eyre::Result<Self> {
        let json_path: PathBuf = VSCodePath::StorageJson.try_into()?;
        debug!("Trying to load storage json from: {}", json_path.display());
        let json = std::fs::read_to_string(&json_path)?;
        let storage_json: Self =
            serde_json::from_str(&json).wrap_err(eyre!("Reading {}", json_path.display()))?;
        Ok(storage_json)
    }
}
