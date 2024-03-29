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


use dioxus::prelude::*;
use crate::{prelude::*, websocket::websocket};
use gloo::storage::{LocalStorage, Storage};

macro_rules! loading_ready {
    ($ready: ident, $page: path) => {
        if *$ready {
            rsx! {
                $crate::$page {}
            }
        } else {
            rsx! {
                components::Loading {}
            }
        }
    };
}


pub fn App(cx: Scope) -> Element {
    use_init_atom_root(cx);

    cx.use_hook(|| cx.provide_context(API_URL));

    let set_user = use_set(cx, USER);

    cx.use_hook(|| {
        let user = LocalStorage::get::<(types::Token, types::ULID)>("user").ok();
        log::info!("{user:?}");
        set_user(user);
    });

    let user_state = use_read(cx, USERS);
    let set_user_state = use_set(cx, USERS);

    let server_state = use_read(cx, SERVERS);
    let set_server_state = use_set(cx, SERVERS);

    let channel_state = use_read(cx, CHANNELS);
    let set_channel_state = use_set(cx, CHANNELS);

    let server_member_state = use_read(cx, SERVER_MEMBERS);
    let set_server_member_state = use_set(cx, SERVER_MEMBERS);

    let message_state = use_read(cx, MESSAGES);
    let set_message_state = use_set(cx, MESSAGES);

    let typing_state = use_read(cx, TYPING);
    let set_typing_state = use_set(cx, TYPING);

    let dm_channel_state = use_read(cx, DM_CHANNELS);
    let set_dm_channel_state = use_set(cx, DM_CHANNELS);

    let emoji_state = use_read(cx, EMOJIS);
    let set_emoji_state = use_set(cx, EMOJIS);

    let http_state = use_read(cx, HTTP);
    let set_http = use_set(cx, HTTP);

    let user = use_read(cx, USER);

    let revolt_config = use_read(cx, REVOLT_CONFIG);
    let set_config = use_set(cx, REVOLT_CONFIG);

    let ready = use_read(cx, READY);
    let set_ready = use_set(cx, READY);

    let set_saved_messages = use_set(cx, SAVED_MESSAGES);

    let theme = use_theme(cx);

    log::info!("{user:?} {revolt_config:?}");

    if revolt_config.is_none() {
        let set_config = set_config.clone();

        cx.spawn(async move {
            let client = reqwest::Client::new();

            let res = client.get(API_URL)
                .send()
                .await
                .unwrap()
                .json::<types::RevoltConfig>()
                .await
                .unwrap();

            set_config(Some(res));
        });

        return cx.render(rsx! {
            components::Loading {}
        })
    }

    if let Some((token, user_id)) = user && let Some(config) = revolt_config && http_state.is_none() {
        LocalStorage::set("user", (token.clone(), user_id.clone())).unwrap();

        to_owned![
            user_state,
            set_user_state,
            server_state,
            set_server_state,
            channel_state,
            set_channel_state,
            server_member_state,
            set_server_member_state,
            message_state,
            set_message_state,
            typing_state,
            set_typing_state,
            dm_channel_state,
            set_dm_channel_state,
            emoji_state,
            set_emoji_state,
            set_ready,
            set_saved_messages
        ];

        let http = HTTPClient::new(token.clone(), user_id.clone(), API_URL, config.clone());
        set_http(Some(http.clone()));

        cx.spawn(async move {
            websocket(
                http,
                user_state.clone(),
                set_user_state.clone(),
                server_state.clone(),
                set_server_state.clone(),
                channel_state.clone(),
                set_channel_state.clone(),
                server_member_state.clone(),
                set_server_member_state.clone(),
                message_state.clone(),
                set_message_state.clone(),
                typing_state.clone(),
                set_typing_state.clone(),
                dm_channel_state.clone(),
                set_dm_channel_state.clone(),
                emoji_state.clone(),
                set_emoji_state.clone(),
                set_saved_messages.clone(),
                set_ready.clone()
            ).await;
        })
    };

    cx.render(rsx!(Router {
        div {
            style: "color: {theme.foreground}; caret-color: {theme.accent}; width: 100%; height: 100%",
            components::ContextMenu {},
            components::Modal {},
            Route {
                to: "/",
                pages::Home {}
            },
            Route {
                to: "/login",
                pages::Login {}
            },
            Route {
                to: "/server/:server_id/channel/:channel_id",
                loading_ready!(ready, pages::Channel)
            }
            Route {
                to: "/channel/:channel_id",
                loading_ready!(ready, pages::DmChannel)
            },
            Route {
                to: "/saved_messages",
                loading_ready!(ready, pages::SavedMessages)
            }
        }
    }))
}
