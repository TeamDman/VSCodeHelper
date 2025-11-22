use serde::Deserialize;
use serde::Serialize;

use crate::storage_json::side::Side;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutInfo {
    pub side_bar_side: Side,
    pub editor_part_min_width: u32,
    pub title_bar_height: u32,
    pub activity_bar_width: u32,
    pub side_bar_width: u32,
    pub auxiliary_side_bar_width: Option<u32>,
    pub status_bar_height: u32,
    pub window_border: bool,
}
