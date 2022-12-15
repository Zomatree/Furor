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


use std::fmt;

use serde::{Deserialize, Serialize};
use crate::prelude::*;


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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            Self::User(token) | Self::Bot(token) => token.clone(),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MfaResponse {
    Password(String),
    RecoveryCode(String),
    TotpCode(String)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum LoginBody {
    Details {
        email: String,
        password: String,
        friendly_name: Option<String>
    },
    Mfa {
        mfa_ticket: String,
        mfa_response: MfaResponse,
        friendly_name: Option<String>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MFAMethod {
    Password,
    Recovery,
    Totp
}

impl fmt::Display for MFAMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Password => "Password",
            Self::Recovery => "Recovery Code",
            Self::Totp => "Totp Code"
        }.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WebPushSubscription {
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "result")]
pub enum Login {
    Success {
        #[serde(rename="_id")]
        id: types::ULID,

        user_id: String,
        token: String,
        name: String,
        subscription: Option<WebPushSubscription>
    },
    #[serde(rename="MFA")]
    Mfa {
        ticket: String,
        allowed_methods: Vec<MFAMethod>
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TooManyRequests {
    pub retry_after: u64
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutumnResponse {
    pub id: types::ULID
}
