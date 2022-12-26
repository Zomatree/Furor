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

pub fn ServerList(cx: Scope) -> Element {
    let set_current_server = use_set(cx, CURRENT_SERVER);
    let set_current_channel = use_set(cx, CURRENT_CHANNEL);
    let server_state = use_read(cx, SERVERS);
    let channel_state = use_read(cx, CHANNELS);
    let server_members_state = use_read(cx, SERVER_MEMBERS);
    let revolt_config = use_config(cx);
    let (_, user_id) = use_read(cx, USER).as_ref().unwrap();
    let user_state = use_read(cx, USERS);
    let theme = use_theme(cx);

    let router = use_router(cx);

    let user = &user_state[user_id];

    let (_, avatar) = get_username_avatar(channel_state, server_members_state, revolt_config, user, &None, None);

    cx.render(rsx!(div {
        style: "display: flex; width: 56px; min-width: 56px; flex-direction: column; justify-content: flex-start; overflow-y: auto; align-items: center; background-color: {theme.background}",
        Link {
            to: "/",
            div {
                style: "background: none; border: none",
                components::Icon {
                    src: avatar,
                    height: 42,
                    width: 42,
                }
            }
        },
        div {
            style: "display: flex; margin: 6px auto; user-select: none; align-items: center; height: 1px; width: calc(100% - 10px); background: {theme.secondary_header}"
        },
        server_state.values().map(|server| {
            let types::Server { icon, id, .. } = server.clone();

            let icon = icon.unwrap().url(&revolt_config.features.autumn.url);
            let key = id.clone();

            rsx! {
                button {
                    style: "background: none; border: none",
                    key: "{key}",
                    onclick: move |_| {
                        set_current_server(Some(id.clone()));

                        let channel = get_last_channel(&id).unwrap_or_else(|| {
                            server_state[&id].channels
                                .iter()
                                .map(|id| &channel_state[id])
                                .find(|channel| matches!(channel, types::Channel::TextChannel { .. }))
                                .unwrap()
                                .id()
                        });

                        set_current_channel(Some(channel.clone()));
                        router.push_route(&format!("/server/{id}/channel/{channel}"), None, None);
                    },
                    components::Icon {
                        src: icon,
                        height: 42,
                        width: 42
                    }
                }
            }
        })
    }))
}
