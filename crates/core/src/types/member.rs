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
pub struct MemberId {
    pub server: ULID,
    pub user: ULID
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberId,

    pub nickname: Option<String>,
    pub avatar: Option<Asset>,

    #[serde(default)]
    pub roles: Vec<String>
}

impl Member {
    pub fn from_ids(server_id: ULID, user_id: ULID) -> Self {
        Self {
            id: MemberId {
                server: server_id,
                user: user_id
            },
            nickname: None,
            avatar: None,
            roles: Vec::new()
        }
    }
}
