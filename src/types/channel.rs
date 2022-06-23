use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::{
    asset::Asset,
    ulid::ULID,
    permissions::PermissionsOverwrite
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SavedMessages {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub user: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DirectMessage {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub active: bool,
    pub recipients: Vec<ULID>,
    pub last_message_id: Option<ULID>
}

impl DirectMessage {
    pub fn get_recipient(&self, current: &ULID) -> &ULID {
        self.recipients
            .iter()
            .find(|&id| id != current)
            .unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Group {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub recipients: Vec<String>,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub last_message_id: Option<String>,
    pub icon: Option<Asset>,
    pub permissions: Option<u64>,

    #[serde(default)]
    pub nsfw: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TextChannel {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub server: ULID,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<Asset>,

    #[serde(default)]
    pub default_permissions: PermissionsOverwrite,

    #[serde(default)]
    pub role_permissions: HashMap<ULID, PermissionsOverwrite>,

    #[serde(default)]
    pub nsfw: bool,

    pub last_message_id: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VoiceChannel {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub server: ULID,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<Asset>,

    #[serde(default)]
    pub default_permissions: PermissionsOverwrite,

    #[serde(default)]
    pub role_permissions: HashMap<String, PermissionsOverwrite>,

    #[serde(default)]
    pub nsfw: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "channel_type")]
pub enum Channel {
    SavedMessages(SavedMessages),
    DirectMessage(DirectMessage),
    Group(Group),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel)
}

impl Channel {
    pub fn id(&self) -> ULID {
        match self {
            Self::SavedMessages(c) => c.id.clone(),
            Self::DirectMessage(c) => c.id.clone(),
            Self::Group(c) => c.id.clone(),
            Self::TextChannel(c) => c.id.clone(),
            Self::VoiceChannel(c) => c.id.clone(),
        }
    }

    pub fn server(&self) -> Option<ULID> {
        match self {
            Self::TextChannel(c) => Some(c.server.clone()),
            Self::VoiceChannel(c) => Some(c.server.clone()),
            _ => None
        }
    }

    pub fn name(&self) -> Option<String> {
        match self {
            Self::TextChannel(c) => Some(c.name.clone()),
            Self::VoiceChannel(c) => Some(c.name.clone()),
            Self::Group(c) => Some(c.name.clone()),
            _ => None
        }
    }
}
