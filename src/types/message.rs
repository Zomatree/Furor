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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum SpecialEmbed {
    None {},
    Youtube {
        id: String,
        timestamp: Option<String>
    },
    Twitch {
        content_type: TwitchContentType,
        id: String
    },
    Spotify {
        content_type: String,
        id: String
    },
    Soundcloud {},
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
#[serde(tag = "type")]
pub enum Embed {
    Website {
        url: Option<String>,
        special: Option<SpecialEmbed>,
        title: Option<String>,
        description: Option<String>,
        image: Option<Image>,
        video: Option<Video>,
        site_name: Option<String>,
        icon_url: Option<String>,
        colour: Option<String>
    }
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
pub struct MessageEdited {
    #[serde(rename = "$date")]
    pub date: String
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

    pub masquerade: Option<Masquerade>
}

impl Message {
    pub fn update(&mut self, data: MessageUpdateData) {
        if let Some(new_content) = data.content {
            self.content = Some(new_content)
        }
    }
}
