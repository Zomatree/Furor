use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::lib::types::{
    role::Role,
    permissions::Permissions,
    asset::Asset
};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub id: String,
    pub title: String,
    pub channels: Vec<String>
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ServerSystemMessages {
    pub user_joined: Option<String>,
    pub user_left: Option<String>,
    pub user_kicked: Option<String>,
    pub user_banned: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: String,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub channels: Vec<String>,

    #[serde(default)]
    pub categories: Vec<Category>,

    #[serde(default)]
    pub system_messages: ServerSystemMessages,

    #[serde(default)]
    pub roles: HashMap<String, Role>,

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
