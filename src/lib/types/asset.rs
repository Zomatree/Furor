use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Attachments,
    Avatars,
    Backgrounds,
    Banners,
    Icons,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum AssetMetadata {
    File {},
    Text {},
    Audio {},
    Image {
        width: u64,
        height: u64
    },
    Video {
        width: u64,
        height: u64
    },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Asset {
    #[serde(rename = "_id")]
    pub id: String,

    pub tag: AssetType,
    pub size: u64,
    pub filename: String,
    pub metadata: AssetMetadata,
    pub content_type: String
}
