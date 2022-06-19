use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{
    asset::Asset,
    ulid::ULID,
    permissions::PermissionsOverwrite
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
        default_permissions: PermissionsOverwrite,

        #[serde(default)]
        role_permissions: HashMap<ULID, PermissionsOverwrite>,

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
        default_permissions: PermissionsOverwrite,

        #[serde(default)]
        role_permissions: HashMap<String, PermissionsOverwrite>,

        #[serde(default)]
        nsfw: bool,
    }
}

impl Channel {
    pub fn id(&self) -> ULID {
        match self {
            Self::SavedMessages { id, .. } => id.clone(),
            Self::DirectMessage { id, .. } => id.clone(),
            Self::Group { id, .. } => id.clone(),
            Self::TextChannel { id, .. } => id.clone(),
            Self::VoiceChannel { id, .. } => id.clone(),
        }
    }

    pub fn server(&self) -> Option<ULID> {
        match self {
            Self::TextChannel { server, ..} | Self::VoiceChannel { server, .. } => Some(server.clone()),
            _ => None
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::TextChannel { name, .. } | Self::VoiceChannel { name, ..} | Self::Group { name, .. } => Some(name.clone()),
            _ => None
        }
    }
}
