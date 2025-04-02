use super::window::Window;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsState {
    pub last_active_window: Window,
    pub last_plugin_development_host_window: Window,
    pub opened_windows: Vec<Window>,
}
