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

use crate::types::ULID;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { id: String },
    Detached,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Emoji {
    #[serde(rename = "_id")]
    pub id: ULID,

    pub parent: EmojiParent,
    pub creator_id: String,
    pub name: String,

    #[serde(default)]
    pub animated: bool,

    #[serde(default)]
    pub nsfw: bool,
}
