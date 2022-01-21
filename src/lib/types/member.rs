use serde::{Deserialize, Serialize};
use crate::lib::types::asset::Asset;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MemberId {
    server: String,
    user: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberId,

    pub nickname: Option<String>,
    pub avatar: Option<Asset>,
    pub roles: Vec<String>
}
