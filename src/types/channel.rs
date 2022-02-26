use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{
    asset::Asset,
    role::Role,
    ulid::ULID,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages {
        #[serde(rename = "_id")]
        id: ULID,

        user: String
    },
    DirectMessage {
        #[serde(rename = "_id")]
        id: ULID,

        active: bool,
        recipients: Vec<String>,
        last_message_id: Option<String>
    },
    Group {
        #[serde(rename = "_id")]
        id: ULID,

        recipients: Vec<String>,
        name: String,
        owner: String,
        description: Option<String>,
        last_message_id: Option<String>,
        icon: Option<Asset>,
        permissions: Option<u64>,

        #[serde(default)]
        nsfw: bool
    },
    TextChannel {
        #[serde(rename = "_id")]
        id: ULID,

        server: ULID,
        name: String,
        description: Option<String>,
        icon: Option<Asset>,

        #[serde(default)]
        default_permissions: u64,

        #[serde(default)]
        role_permissions: HashMap<ULID, u64>,

        #[serde(default)]
        nsfw: bool,

        last_message_id: Option<String>
    },
    VoiceChannel {
        #[serde(rename = "_id")]
        id: ULID,

        server: ULID,
        name: String,
        description: Option<String>,
        icon: Option<Asset>,

        #[serde(default)]
        default_permissions: u64,

        #[serde(default)]
        role_permissions: HashMap<String, Role>,

        #[serde(default)]
        nsfw: bool,
    }
}

impl Channel {
    pub fn id(&self) -> ULID {
        match self {
            Channel::SavedMessages { id, .. } => id.clone(),
            Channel::DirectMessage { id, .. } => id.clone(),
            Channel::Group { id, .. } => id.clone(),
            Channel::TextChannel { id, .. } => id.clone(),
            Channel::VoiceChannel { id, .. } => id.clone(),
        }
    }

    pub fn server(&self) -> Option<ULID> {
        match self {
            Channel::SavedMessages { .. } => None,
            Channel::DirectMessage { .. } => None,
            Channel::Group { .. } => None,
            Channel::TextChannel { server, ..} => Some(server.clone()),
            Channel::VoiceChannel { server, .. } => Some(server.clone()),
        }
    }
}
