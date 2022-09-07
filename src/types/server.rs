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
    role::Role,
    asset::Asset,
    ulid::ULID,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub id: ULID,
    pub title: String,
    pub channels: Vec<ULID>
}

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct ServerSystemMessages {
    pub user_joined: Option<ULID>,
    pub user_left: Option<ULID>,
    pub user_kicked: Option<ULID>,
    pub user_banned: Option<ULID>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id")]
    pub id: ULID,
    pub owner: String,
    pub name: String,
    pub description: Option<String>,
    pub channels: Vec<ULID>,

    #[serde(default)]
    pub categories: Vec<Category>,

    #[serde(default)]
    pub system_messages: ServerSystemMessages,

    #[serde(default)]
    pub roles: HashMap<ULID, Role>,

    pub default_permissions: u64,
    pub icon: Option<Asset>,
    pub banner: Option<Asset>,

    #[serde(default)]
    pub nsfw: bool,

    #[serde(default)]
    pub flags: u8,

    #[serde(default)]
    pub analytics: bool,

    #[serde(default)]
    pub discoverable: bool
}
