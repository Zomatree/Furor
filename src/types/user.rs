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
use crate::types::{asset::Asset, ulid::ULID};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum RelationStatus {
    Blocked,
    BlockedOther,
    Friend,
    Incoming,
    None,
    Outgoing,
    User
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserRelation {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub status: RelationStatus
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum UserPresence {
    Busy,
    Idle,
    #[serde(rename = "Invisible")]
    Offline,
    Online
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserStatus {
    pub text: Option<String>,
    #[serde(default = "offline_presence")]
    pub presence: UserPresence
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Bot {
    pub owner: ULID,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ULID,
    pub username: String,
    pub avatar: Option<Asset>,

    #[serde(default)]
    pub relations: Vec<UserRelation>,

    #[serde(default)]
    pub badges: u16,

    pub status: Option<UserStatus>,

    #[serde(default = "no_relation")]
    pub relationship: RelationStatus,

    #[serde(default)]
    pub online: bool,

    #[serde(default)]
    pub flags: u8,

    pub bot: Option<Bot>
}

const fn no_relation() -> RelationStatus {
    RelationStatus::None
}

const fn offline_presence() -> UserPresence {
    UserPresence::Offline
}
