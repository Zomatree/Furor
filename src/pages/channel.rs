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

pub fn Channel(cx: Scope) -> Element {
    redirect_to_login(cx);

    let route = use_route(cx);
    let bump = use_alloc(cx);
    let theme = use_theme(cx);

    let server_id = bump.alloc(route.parse_segment::<types::ULID>("server_id").unwrap().unwrap());
    let channel_id = bump.alloc(route.parse_segment::<types::ULID>("channel_id").unwrap().unwrap());

    cx.render(rsx!(div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: row; background-color: {theme.secondary_background}",
        components::ServerList {},
        div {
            style: "display: flex; flex-direction: row; flex-grow: 1",
            div {
                style: "display: flex; flex-direction: column; flex-grow: 1",
                components::ServerHeader {
                    server_id: server_id,
                }
                components::ChannelList {
                    server_id: server_id
                },
            },
            components::Channel {
                channel_id: channel_id
            },
            // components::MemberList {
            //     channel_id: channel_id,
            //     server_id: server_id
            // }
        }
    }))
}
