use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProfileDunder {
    #[serde(rename="__default__profile__")]
    DefaultProfile
}