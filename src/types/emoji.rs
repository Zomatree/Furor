use serde::{Deserialize, Serialize};

use crate::types::ULID;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: String },
    Detached,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub parent: EmojiParent,
    pub creator_id: String,
    pub name: String,

    #[serde(default)]
    pub animated: bool,

    #[serde(default)]
    pub nsfw: bool,
}
