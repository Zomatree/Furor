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


use serde::{Deserialize, Serialize};
use crate::types::ulid::ULID;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AssetType {
    Attachments,
    Avatars,
    Backgrounds,
    Banners,
    Icons,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attachments => "attachments",
            Self::Avatars => "avatars",
            Self::Backgrounds => "backgrounds",
            Self::Banners => "banners",
            Self::Icons => "icons",

        }.fmt(f)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum AssetMetadata {
    File,
    Text,
    Audio,
    Image {
        width: u64,
        height: u64
    },
    Video {
        width: u64,
        height: u64
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Asset {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub tag: AssetType,
    pub size: u64,
    pub filename: String,
    pub metadata: AssetMetadata,
    pub content_type: String,

    #[serde(default)]
    pub default: bool,  // hacky solution to reuse this for default avatars as well
}

impl Asset {
    pub fn url(&self, autumn_url: &str, api_url: &str) -> String {
        if self.default {
            format!("{api_url}/users/{}/default_avatar", self.id)
        } else {
            format!("{autumn_url}/{}/{}/{}", self.tag, self.id, self.filename)
        }
    }

    pub fn as_default_avatar(user_id: ULID) -> Self {
        Self {
            id: user_id,
            tag: AssetType::Avatars,
            size: 0,
            filename: String::new(),
            metadata: AssetMetadata::File {  },
            content_type: String::new(),
            default: true
        }
    }

    pub fn resolution(&self) -> (Option<u64>, Option<u64>) {
        match self.metadata {
            AssetMetadata::File {  } => (None, None),
            AssetMetadata::Text {  } => (None, None),
            AssetMetadata::Audio {  } => (None, None),
            AssetMetadata::Image { width, height } => (Some(width), Some(height)),
            AssetMetadata::Video { width, height } => (Some(width), Some(height)),
        }
    }
}
