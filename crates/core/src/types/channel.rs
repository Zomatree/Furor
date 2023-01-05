/* Copyright (C) 2022-current  Zomatree <me@zomatree.live>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/. */


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
    pub fn id(&self) -> &ULID {
        match self {
            Self::SavedMessages(c) => &c.id,
            Self::DirectMessage(c) => &c.id,
            Self::Group(c) => &c.id,
            Self::TextChannel(c) => &c.id,
            Self::VoiceChannel(c) => &c.id,
        }
    }

    pub fn server(&self) -> Option<&ULID> {
        match self {
            Self::TextChannel(c) => Some(&c.server),
            Self::VoiceChannel(c) => Some(&c.server),
            _ => None
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::TextChannel(c) => Some(&c.name),
            Self::VoiceChannel(c) => Some(&c.name),
            Self::Group(c) => Some(&c.name),
            Self::SavedMessages(_) => Some("Saved Messages"),
            _ => None
        }
    }
}
