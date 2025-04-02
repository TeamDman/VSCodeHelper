use serde::Deserialize;
use serde::Serialize;

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
