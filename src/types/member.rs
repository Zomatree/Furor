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

impl Member {
    pub fn from_ids(server_id: ULID, user_id: ULID) -> Self {
        Self {
            id: MemberId {
                server: server_id,
                user: user_id
            },
            nickname: None,
            avatar: None,
            roles: Vec::new()
        }
    }
}
