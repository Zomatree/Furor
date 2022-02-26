use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::{
    role::Role,
    permissions::Permissions,
    asset::Asset,
    ulid::ULID,
};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub id: ULID,
    pub title: String,
    pub channels: Vec<ULID>
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ServerSystemMessages {
    pub user_joined: Option<ULID>,
    pub user_left: Option<ULID>,
    pub user_kicked: Option<ULID>,
    pub user_banned: Option<ULID>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: ULID,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub channels: Vec<ULID>,

    #[serde(default)]
    pub categories: Vec<Category>,

    #[serde(default)]
    pub system_messages: ServerSystemMessages,

    #[serde(default)]
    pub roles: HashMap<ULID, Role>,

    pub default_permissions: Permissions,
    pub icon: Option<Asset>,
    pub banner: Option<Asset>,

    #[serde(default)]
    pub nsfw: bool,

    #[serde(default)]
    pub flags: u8,

    #[serde(default)]
    pub analytics: bool,

    #[serde(default)]
    pub discoverable: bool
}
