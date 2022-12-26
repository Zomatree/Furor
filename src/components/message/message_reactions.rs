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
pub struct MessageReactionsProps<'a> {
    message_id: &'a types::ULID,
    channel_id: &'a types::ULID
}

#[derive(Props, PartialEq)]
pub struct MessageReactionProps<'a> {
    channel_id: &'a types::ULID,
    message_id: &'a types::ULID,
    emoji: String,
    count: usize,
    reacted: bool,
}

pub fn MessageReaction<'a>(cx: Scope<'a, MessageReactionProps<'a>>) -> Element<'a> {
    let http = use_http(cx);

    cx.render(rsx! {
        button {
            onclick: move |_| {
                let http = http.clone();
                let channel_id = cx.props.channel_id.clone();
                let message_id = cx.props.message_id.clone();
                let emoji = cx.props.emoji.clone();
                let reacted = cx.props.reacted;

                cx.spawn(async move {
                    if reacted {
                        http.remove_reaction(&channel_id, &message_id, &emoji).await;
                    } else {
                        http.add_reaction(&channel_id, &message_id, &emoji).await;
                    }
                });
            },
            border_color: format_args!("{}", if cx.props.reacted { "yellow" } else { "buttonborder" }),
            key: "{cx.props.emoji}",
            "{cx.props.emoji} {cx.props.count}",
        }
    })
}

pub fn MessageReactions<'a>(cx: Scope<'a, MessageReactionsProps<'a>>) -> Element<'a> {
    let message_state = use_read(cx, MESSAGES);
    let user_id = use_user(cx).1;

    let message = &message_state[cx.props.channel_id][cx.props.message_id];
    let message_reactions = &message.reactions;

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: row",
            message_reactions.iter().map(|(emoji, users)| rsx! {
                MessageReaction {
                    emoji: emoji.clone(),
                    channel_id: cx.props.channel_id,
                    message_id: cx.props.message_id,
                    count: users.len(),
                    reacted: users.contains(user_id)
                }
            })
        }
    })
}
