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


use crate::prelude::*;

mod modal;
mod login;
mod message;
mod channel;
mod files;
mod context_menu;
mod r#async;
mod theme;

use dioxus::core::exports::bumpalo::Bump;
pub use modal::*;
pub use login::*;
pub use message::*;
pub use channel::*;
pub use files::*;
pub use context_menu::*;
pub use r#async::*;
pub use theme::*;

pub fn get_username_avatar(
    channels: &ChannelState,
    server_members: &ServerMemberState,
    revolt_config: &types::RevoltConfig,
    user: &types::User,
    masquerade: &Option<types::Masquerade>,
    channel_id: Option<&types::ULID>,
    api_url: &str
) -> (String, String) {
    match masquerade {
        Some(mask) => (
            mask.name.clone().unwrap_or_else(|| user.username.clone()),
            mask.avatar
                .clone()
                .unwrap_or_else(|| user.avatar.clone().unwrap().url(&revolt_config.features.autumn.url, api_url)),
        ),
        None => {
            let server = channel_id
                .and_then(|id| channels.get(id))
                .and_then(|channel| channel.server());

            let default_avatar = types::Asset::as_default_avatar(user.id.clone());

            match server {
                Some(server_id) => {
                    let member = server_members.get(&server_id)
                        .unwrap()
                        .get(&user.id)
                        .unwrap();


                    (
                        member
                            .nickname
                            .clone()
                            .unwrap_or_else(|| user.username.clone()),
                        member
                            .avatar
                            .as_ref()
                            .or(user.avatar.as_ref())
                            .unwrap_or(&default_avatar)
                            .url(&revolt_config.features.autumn.url, api_url),
                    )
                }
                None => (
                    user.username.clone(),
                    user.avatar.clone().unwrap_or(default_avatar).url(&revolt_config.features.autumn.url, api_url),
                ),
            }
        }
    }
}


pub fn format_datetime<Tz: chrono::TimeZone>(dt: &chrono::DateTime<Tz>) -> String where Tz::Offset: std::fmt::Display {
    let now = chrono::Utc::now();
    let yesterday = now - chrono::Duration::days(1);

    if dt.date_naive() == now.date_naive() {
        dt.format("Today at %H:%M").to_string()
    } else if dt.date_naive() == yesterday.date_naive() {
        dt.format("Yesterday at %H:%M").to_string()
    } else {
        dt.format("%d/%m/%Y").to_string()
    }
}

pub fn use_http(cx: &ScopeState) -> &HTTPClient {
    use_read(cx, HTTP).as_ref().unwrap()
}

pub fn use_config(cx: &ScopeState) -> &types::RevoltConfig {
    use_read(cx, REVOLT_CONFIG).as_ref().unwrap()
}

pub fn use_user(cx: &ScopeState) -> (&types::Token, &types::ULID) {
    let user = use_read(cx, USER).as_ref().unwrap();
    (&user.0, &user.1)
}

pub fn use_theme(cx: &ScopeState) -> &Theme {
    use_read(cx, THEME)
}

pub fn use_alloc(cx: &ScopeState) -> &Bump {
    cx.bump()
}

pub fn use_api(cx: &ScopeState) -> &str {
    cx.consume_context().unwrap()
}

#[macro_export]
macro_rules! move_variables {
    ($($variable: ident),+) => {
        $(
            #[allow(unused_mut)]
            let mut $variable = $variable;
        )+
    };
}
