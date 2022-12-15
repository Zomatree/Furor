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

#[derive(Props, PartialEq)]
pub struct ChannelListProps<'a> {
    server_id: &'a types::ULID
}

pub fn ChannelList<'a>(cx: Scope<'a, ChannelListProps<'a>>) -> Element<'a> {
    let server_state = use_read(cx, SERVERS);
    let channel_state = use_read(cx, CHANNELS);
    let set_channel = use_set(cx, CURRENT_CHANNEL);
    let router = use_router(cx);

    cx.render(rsx!(div {
        style: "display: flex; flex-direction: column; width: 232px",
        server_state[cx.props.server_id]
            .categories
            .iter()
            .map(|category| {
                rsx! {
                    "{category.title}",
                    category.channels.iter()
                    .filter_map(|channel_id| channel_state.get(channel_id))
                    .map(|channel| {
                        match channel {
                            types::Channel::TextChannel(channel) => {
                                let cloned_id = channel.id.clone();

                                rsx! {
                                    button {
                                        key: "{cloned_id}",
                                        style: "display: flex; flex-direction: row",
                                        onclick: move |_| {
                                            set_channel(Some(channel.id.clone()));
                                            set_last_channel(cx.props.server_id.clone(), channel.id.clone());

                                            router.push_route(&format!("/server/{}/channel/{}", cx.props.server_id, channel.id), None, None);
                                        },
                                        span {
                                            "# ",
                                            "{channel.name}"
                                        },
                                    }
                                }
                            },
                            types::Channel::VoiceChannel(channel) => {
                                rsx! {
                                    button {
                                        key: "{channel.id}",
                                        style: "display: flex; flex-direction: row",
                                        span {
                                            "V ",
                                            "{channel.name}"
                                        },
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    })
                }
            })
    }))
}
