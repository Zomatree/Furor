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
pub struct ChannelProps<'a> {
    pub channel_id: &'a types::ULID,
}

pub fn Channel<'a>(cx: Scope<'a, ChannelProps<'a>>) -> Element<'a> {
    let channel_state = use_read(cx, CHANNELS);
    let user_state = use_read(cx, USERS);

    let (_, user_id) = use_read(cx, USER).as_ref().unwrap();

    let name = match &channel_state[cx.props.channel_id] {
        types::Channel::DirectMessage(dm) => {
            let recipient_id = dm.get_recipient(user_id);
            &user_state[recipient_id].username
        },
        channel => channel.name().unwrap()
    };

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%",
            div {
                style: "height: 48px; width: 100%",
                "{name}"
            }
            components::ChannelMessages {
                channel_id: cx.props.channel_id,
            },
            components::Typing {
                channel_id: cx.props.channel_id,

            }
            components::MessageArea {
                channel_id: cx.props.channel_id,
            }
        }
    })
}
