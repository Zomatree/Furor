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


use im_rc::HashMap;

use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelProps<'a> {
    channel_id: &'a types::ULID
}

pub fn ChannelMessages<'a>(cx: Scope<'a, ChannelProps<'a>>) -> Element<'a> {
    let message_state = use_read(cx, MESSAGES);
    let default = cx.use_hook(HashMap::new);

    let mut messages = message_state
        .get(cx.props.channel_id)
        .unwrap_or(default)
        .values()
        .collect::<Vec<_>>();

    messages.sort_by(|&a, &b| a.id.timestamp().cmp(&b.id.timestamp()));

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; flex-grow: 1; overflow: hidden scroll; padding-bottom: 26px; justify-content: flex-end;",
            messages.iter().map(|message| {
                rsx! {
                    components::Message {
                        key: "{message.id}",
                        channel_id: &message.channel,
                        message_id: &message.id,
                    }
                }
            })
        }
    })
}
