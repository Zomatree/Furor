use serde::{Deserialize, Serialize};
use crate::lib::types::asset::Asset;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum RelationStatus {
    Blocked,
    BlockedOther,
    Friend,
    Incoming,
    None,
    Outgoing,
    User
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserRelation {
    #[serde(rename = "_id")]
    pub id: String,

    pub status: RelationStatus
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserPresence {
    Busy,
    Idle,
    #[serde(rename = "Invisible")]
    Offline,
    Online
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserStatus {
    pub text: Option<String>,
    #[serde(default = "offline_presence")]
    pub presence: UserPresence
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bot {
    pub owner: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
    pub avatar: Option<Asset>,

    #[serde(default)]
    pub relations: Vec<UserRelation>,

    #[serde(default)]
    pub badges: u16,

    pub status: Option<UserStatus>,

    #[serde(default = "no_relation")]
    pub relationship: RelationStatus,

    #[serde(default)]
    pub online: bool,

    #[serde(default)]
    pub flags: u8,

    pub bot: Option<Bot>
}

fn no_relation() -> RelationStatus {
    RelationStatus::None
}

fn offline_presence() -> UserPresence {
    UserPresence::Offline
}
