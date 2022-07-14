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

