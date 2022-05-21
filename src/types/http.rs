use serde::{Deserialize, Serialize};
use crate::prelude::*;


#[derive(Deserialize, Debug, Clone)]
pub struct Login {
    #[serde(rename="_id")]
    pub id: types::ULID,

    pub user_id: String,
    pub token: String,
    pub name: String,
    pub subscription: String
}

#[derive(Deserialize, Serialize, Default)]
pub struct SendMessage {
    #[serde(skip_serializing_if="Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    pub attachments: Vec<types::ULID>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    pub embeds: Vec<types::SendableEmbed>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    pub replies: Vec<types::Reply>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub masquerade: Option<types::Masquerade>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SendableEmbed {
    pub icon_url: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
	pub colour: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    User(String),
    Bot(String)
}

impl Token {
    pub fn to_header(&self) -> (&'static str, String) {
        match self {
            Token::User(token) => ("x-session-token", token.clone()),
            Token::Bot(token) => ("x-bot-token", token.clone())
        }
    }

    pub fn inner(&self) -> String {
        match self {
            Token::User(token) => token.clone(),
            Token::Bot(token) => token.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ServerMembers {
    pub members: Vec<types::Member>,
    pub users: Vec<types::User>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CaptchaFeature {
    pub enabled: bool,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Feature {
    pub enabled: bool,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct VoiceFeature {
    pub enabled: bool,
    pub url: String,
    pub ws: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RevoltFeatures {
    pub captcha: CaptchaFeature,
    pub email: bool,
    pub invite_only: bool,
    pub autumn: Feature,
    pub january: Feature,
    pub voso: VoiceFeature,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RevoltConfig {
    pub revolt: String,
    pub features: RevoltFeatures,
    pub ws: String,
    pub app: String,
    pub vapid: String,
}
