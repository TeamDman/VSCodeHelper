pub mod keys;
pub mod models;
pub mod schema;
#[allow(clippy::module_inception)]
mod state_vscdb;

pub use state_vscdb::VSCodeStateVscdb;

pub mod well_known_keys {
    pub use super::keys::history_recently_opened_paths_list::HistoryRecentlyOpenedPathsListKey;
}