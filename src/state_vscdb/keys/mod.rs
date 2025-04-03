use serde::de::DeserializeOwned;

mod history_recently_opened_paths_list;

pub trait Key {
    const KEY: &'static str;
    type Value: DeserializeOwned;
}

pub mod well_known_keys {
    pub use super::history_recently_opened_paths_list::HistoryRecentlyOpenedPathsListKey;
}