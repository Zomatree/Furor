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
pub struct MemberListProps<'a> {
    pub channel_id: &'a types::ULID,
    pub server_id: &'a types::ULID
}

pub fn MemberList<'a>(cx: Scope<'a, MemberListProps<'a>>) -> Element<'a> {
    let channel_state = use_read(cx, CHANNELS);
    let member_state = use_read(cx, SERVER_MEMBERS);
    let user_state = use_read(cx, USERS);
    let revolt_config = use_config(cx);
    let api_url = use_api(cx);

    let member_list = &member_state[cx.props.server_id];
    cx.render(rsx! {
        div {
            style: "width: 232px; height: 100%; overflow-y: auto",

            member_list.keys().map(|member_id| {
                let user = &user_state[member_id];
                let (username, avatar) = utils::get_username_avatar(channel_state, member_state, revolt_config, user, &None, Some(cx.props.channel_id), api_url);

                rsx! {
                    div {
                        key: "{member_id}",
                        components::Icon {
                            src: avatar
                        },
                        span {
                            "{username}"
                        }
                    }
                }
            })
        }
    })
}
