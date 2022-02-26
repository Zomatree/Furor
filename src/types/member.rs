use serde::{Deserialize, Serialize};
use crate::types::{asset::Asset, ulid::ULID};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct MemberId {
    pub server: ULID,
    pub user: ULID
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberId,

    pub nickname: Option<String>,
    pub avatar: Option<Asset>,

    #[serde(default)]
    pub roles: Vec<String>
}
