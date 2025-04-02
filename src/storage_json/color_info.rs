use crate::storage_json::color::Color;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorInfo {
    pub foreground: Color,
    pub background: Color,
    pub editor_background: Color,
    pub title_bar_background: Color,
    pub title_bar_border: Color,
    pub activity_bar_background: Color,
    pub activity_bar_border: Color,
    pub side_bar_background: Color,
    pub side_bar_border: Color,
    pub status_bar_background: Color,
    pub status_bar_border: Color,
    pub status_bar_no_folder_background: Color,
}
