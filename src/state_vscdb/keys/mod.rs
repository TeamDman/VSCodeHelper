use serde::de::DeserializeOwned;

pub mod history_recently_opened_paths_list;

pub trait Key {
    const KEY: &'static str;
    type Value: DeserializeOwned;
}
