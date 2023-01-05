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
use crate::types::permissions::PermissionsOverwrite;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: PermissionsOverwrite,
    pub colour: Option<String>,

    #[serde(default)]
    pub hoist: bool,

    #[serde(default)]
    pub rank: i64
}
