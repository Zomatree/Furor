use serde::{Deserialize, Serialize};

use super::asset::Asset;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TwitchContentType {
    Channel,
    Clip,
    Video
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum BandcampContentType {
    Album,
    Track
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ImageSize {
    Large,
    Preview
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub width: u16,
    pub height: u16,
    pub size: ImageSize
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Reply {
    pub id: String,
    pub mention: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Masquerade {
    pub name: Option<String>,
    pub avatar: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageEdited {
    #[serde(rename = "$date")]
    pub date: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: String,

    pub nonce: Option<String>,
    pub channel: String,
    pub author: String,
    pub content: String,

    #[serde(default)]
    pub attachments: Vec<Asset>,

    pub edited: Option<MessageEdited>,

    #[serde(default)]
    pub embeds: Vec<Embed>,

    #[serde(default)]
    pub mentions: Vec<String>,

    #[serde(default)]
    pub replies: Vec<Reply>,

    pub masquerade: Option<Masquerade>
}
