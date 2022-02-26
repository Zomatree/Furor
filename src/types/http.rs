use serde::{Deserialize, Serialize};
use crate::types::{ulid::ULID, message::{Masquerade, Reply}, user::User, member::Member};


#[derive(Deserialize, Debug, Clone)]
pub struct Login {
    #[serde(rename="_id")]
    pub id: ULID,

    pub user_id: String,
    pub token: String,
    pub name: String,
    pub subscription: String
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct SendMessage {
    content: String,

    #[serde(skip_serializing_if="Vec::is_empty")]
    attachments: Vec<ULID>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    embeds: Vec<SendableEmbed>,

    #[serde(skip_serializing_if="Vec::is_empty")]
    replies: Vec<Reply>,

    #[serde(skip_serializing_if="Option::is_none")]
    masquerade: Option<Masquerade>
}

impl SendMessage {
    pub fn with_content(content: String) -> Self {
        SendMessage {
            content,
            ..SendMessage::default()
        }
    }
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
    pub members: Vec<Member>,
    pub users: Vec<User>
}
