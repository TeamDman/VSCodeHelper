use crate::storage_json::color_info::ColorInfo;
use crate::storage_json::layout_info::LayoutInfo;
use crate::storage_json::theme::Theme;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSplash {
    pub zoom_level: u8,
    pub base_theme: Theme,
    pub color_info: ColorInfo,
    pub layout_info: LayoutInfo,
}
