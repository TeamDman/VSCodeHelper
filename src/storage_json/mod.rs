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
#[allow(clippy::module_inception)]
mod storage_json;
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

pub use storage_json::VSCodeStorageJson;