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


use gloo::storage::{LocalStorage, Storage};
use im_rc::HashMap;
use crate::prelude::*;


pub fn get_last_channel(server_id: &types::ULID) -> Option<types::ULID> {
    let last_channels = match LocalStorage::get::<HashMap<types::ULID, types::ULID>>("last_channels") {
        Ok(channels) => channels,
        Err(_) => {
            LocalStorage::set("last_channels", HashMap::<types::ULID, types::ULID>::new()).unwrap();
            HashMap::new()
        }
    };

    last_channels
        .get(server_id)
        .cloned()
}

pub fn set_last_channel(server_id: types::ULID, channel_id: types::ULID) {
    let mut last_channels = LocalStorage::get::<HashMap<types::ULID, types::ULID>>("last_channels").unwrap_or_default();

    last_channels.insert(server_id, channel_id);

    LocalStorage::set("last_channels", last_channels).unwrap();
}

