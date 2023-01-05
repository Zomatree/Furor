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


use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::types::{asset::Asset, ulid::ULID, ws::MessageUpdateData};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum TwitchContentType {
    Channel,
    Clip,
    Video
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum BandcampContentType {
    Album,
    Track
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LightspeedType {
    Channel,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum SpecialEmbed {
    None,
    GIF,
    Youtube {
        id: String,
        timestamp: Option<String>
    },
    Lightspeed {
        content_type: LightspeedType,
        id: String
    },
    Twitch {
        content_type: TwitchContentType,
        id: String
    },
    Spotify {
        content_type: String,
        id: String
    },
    Soundcloud,
    Bandcamp {
        content_type: BandcampContentType,
        id: String
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ImageSize {
    Large,
    Preview
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Image {
    pub url: String,
    pub width: u16,
    pub height: u16,
    pub size: ImageSize
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Video {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Text {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Asset>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    original_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    special: Option<SpecialEmbed>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<Video>,

    #[serde(skip_serializing_if = "Option::is_none")]
    site_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    colour: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Embed {
    Website(Metadata),
    Image(Image),
    Video(Video),
    Text(Text),
    None
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Reply {
    pub id: ULID,
    pub mention: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Masquerade {
    pub name: Option<String>,
    pub avatar: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Interactions {
    pub reactions: Option<HashSet<String>>,

    #[serde(default)]
    pub restrict_reactions: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub nonce: Option<String>,
    pub channel: ULID,
    pub author: ULID,
    pub content: Option<String>,

    #[serde(default)]
    pub attachments: Vec<Asset>,

    pub edited: Option<String>,

    #[serde(default)]
    pub embeds: Vec<Embed>,

    #[serde(default)]
    pub mentions: Vec<String>,

    #[serde(default)]
    pub replies: Vec<ULID>,

    pub masquerade: Option<Masquerade>,

    #[serde(default)]
    pub reactions: HashMap<String, HashSet<ULID>>,

    pub interactions: Option<Interactions>
}

impl Message {
    pub fn update(&mut self, data: MessageUpdateData) {
        if let Some(content) = data.content {
            self.content = Some(content);
        }

        if let Some(embeds) = data.embeds {
            self.embeds = embeds;
        }

        self.edited = Some(data.edited);
    }
}
