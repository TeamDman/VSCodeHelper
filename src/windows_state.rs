use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::uri::Uri;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsState {
    pub last_active_window: Window,
    pub last_plugin_development_host_window: Window,
    pub opened_windows: Vec<Window>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Window {
    pub folder: Uri,
    pub backup_path: Option<PathBuf>,
    pub ui_state: UiState,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    pub mode: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub zoom_level: Option<u8>,
}
