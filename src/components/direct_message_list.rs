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

#[derive(Clone)]
enum Channel<'a> {
    Dm(&'a types::DirectMessage),
    Group(&'a types::Group),
}

pub fn DirectMessageList(cx: Scope) -> Element {
    let channels_state = use_read(cx, CHANNELS);
    let dm_channels_state = use_read(cx, DM_CHANNELS);
    let user_state = use_read(cx, USERS);
    let user_id = &use_read(cx, USER).as_ref().unwrap().1;
    let server_members_state = use_read(cx, SERVER_MEMBERS);
    let revolt_config = use_config(cx);

    let router = use_router(cx);

    cx.render(rsx!(div {
        style: "width: 232px; height: 100%; display: flex; flex-direction: column",
        h3 {
            "Direct Messages"
        },
        button {
            onclick: move |_| {
                router.push_route("/", None, None);
            },
            "Home"
        },
        button {
            "Friends"
        },
        button {
            onclick: move |_| {
                router.push_route("/saved_messages", None, None);
            },
            "Saved Notes"
        },
        h3 {
            "Conversations"
        },
        div {
            style: "flex-grow: 1; overflow-y: auto; display: flex; flex-direction: column",
            dm_channels_state.iter()
                .filter_map(|channel| {
                    let channel = channels_state.get(channel)?;
                    match channel {
                        types::Channel::DirectMessage(channel) => Some(Channel::Dm(channel)),
                        types::Channel::Group(channel) => Some(Channel::Group(channel)),
                        _ => None
                    }
                })
                .map(|channel| {
                    let match_channel = channel.clone();
                    let id = match channel {
                        Channel::Dm(dm) => &dm.id,
                        Channel::Group(group) => &group.id
                    };

                    rsx!{
                        button {
                            key: "{id}",
                            style: "gap: 8px; text-align: start;",
                            onclick: move |_| {
                                router.push_route(&format!("/channel/{id}"), None, None);
                            },
                            match match_channel {
                                Channel::Dm(dm) => {
                                    let user_id = dm.get_recipient(user_id);
                                    let user = user_state.get(user_id).unwrap();
                                    let (username, avatar) = get_username_avatar(channels_state, server_members_state, revolt_config, user, &None, None);

                                    rsx! {
                                        Fragment {
                                            components::Icon {
                                                src: avatar
                                            },
                                            span {
                                                "{username}"
                                            }
                                        }
                                    }
                                },
                                Channel::Group(group) => {

                                    rsx! {
                                        Fragment {
                                            group.icon.as_ref().map(|icon| {
                                                let url = icon.url(&revolt_config.features.autumn.url);

                                                rsx! {
                                                    components::Icon {
                                                        src: url
                                                    }
                                                }
                                            }),
                                            span {
                                                "{group.name}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                })
        }
    }))
}
