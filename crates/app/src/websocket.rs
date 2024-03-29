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


use futures::{SinkExt, StreamExt, join, channel::oneshot};
use std::time::Duration;
use ws_stream_wasm::{WsMessage, WsMeta};
use crate::prelude::*;
use std::rc::Rc;
use async_std::{sync::RwLock, task::sleep};
use im_rc::HashMap;

#[allow(clippy::too_many_arguments)]
pub async fn websocket(
    http: HTTPClient,
    mut user_state: UserState,
    set_user_state: FermiSetter<UserState>,
    mut server_state: ServerState,
    set_server_state: FermiSetter<ServerState>,
    mut channel_state: ChannelState,
    set_channel_state: FermiSetter<ChannelState>,
    mut server_member_state: ServerMemberState,
    set_server_member_state: FermiSetter<ServerMemberState>,
    mut message_state: MessageState,
    set_message_state: FermiSetter<MessageState>,
    mut typing_state: TypingState,
    set_typing_state: FermiSetter<TypingState>,
    mut dm_channel_state: DmChannelState,
    set_dm_channel_state: FermiSetter<DmChannelState>,
    mut emoji_state: EmojiState,
    set_emoji_state: FermiSetter<EmojiState>,
    set_saved_messages: FermiSetter<Option<types::SavedMessages>>,
    set_ready: FermiSetter<bool>
) {
    let (_, ws) = WsMeta::connect(http.revolt_config.ws.clone(), None)
        .await
        .unwrap();

    let ws = Rc::new(RwLock::new(ws));

    let bg_ws = ws.clone();

    let (ready_tx, ready_rx) = oneshot::channel();

    let task1 = async move {
        ready_rx.await.unwrap();
        set_ready(true);

        loop {
            bg_ws.write().await.send(WsMessage::Text(serde_json::to_string(&types::SendWsMessage::Ping { data: 0 }).unwrap())).await.unwrap();
            sleep(Duration::from_secs(30)).await;
        }
    };
    let task2 = async move {
        ws.write().await.send(WsMessage::Text(
            serde_json::to_string(&types::SendWsMessage::Authenticate {
                token: http.token.inner().to_string(),
            })
            .unwrap(),
        ))
        .await
        .unwrap();

        let mut ready_tx = Some(ready_tx);

        while let Some(WsMessage::Text(payload)) = ws.write().await.next().await {
            //log::debug!("EVENT RECEIVED {payload}");

            match serde_json::from_str::<types::ReceiveWsMessage>(&payload) {
                Ok(event) => match event {
                    types::ReceiveWsMessage::Authenticated => {

                    }
                    types::ReceiveWsMessage::Ready {
                        users,
                        servers,
                        channels,
                        emojis
                    } => {
                        for user in users {
                            user_state.insert(user.id.clone(), user);
                        }
                        set_user_state(user_state.clone());

                        for server in servers.clone() {
                            server_state.insert(server.id.clone(), server);
                        };

                        set_server_state(server_state.clone());

                        for channel in channels {
                            if let types::Channel::SavedMessages(saved_messages) = &channel {
                                set_saved_messages(Some(saved_messages.clone()))
                            };

                            if matches!(channel, types::Channel::DirectMessage { .. } | types::Channel::Group { .. } ) {
                                dm_channel_state.insert(channel.id().clone());
                            }

                            channel_state.insert(channel.id().clone(), channel);
                        }

                        set_channel_state(channel_state.clone());
                        set_dm_channel_state(dm_channel_state.clone());

                        for server in servers {
                            let types::ServerMembers { users, members } = http.fetch_server_members(&server.id).await;

                            for user in users {
                                user_state.insert(user.id.clone(), user);
                            }

                            let mut current_server_members = HashMap::new();

                            for member in members {
                                current_server_members.insert(member.id.user.clone(), member);
                            }

                            server_member_state.insert(server.id.clone(), current_server_members);
                        };

                        set_user_state(user_state.clone());
                        set_server_member_state(server_member_state.clone());

                        for emoji in emojis {
                            emoji_state.insert(emoji.id.clone(), emoji);
                        };

                        set_emoji_state(emoji_state.clone());

                        ready_tx.take().unwrap().send(()).unwrap();
                    },
                    types::ReceiveWsMessage::Message { message } => {
                        message_state
                            .entry(message.channel.clone())
                            .or_default()
                            .insert(message.id.clone(), message);

                        set_message_state(message_state.clone());
                    },
                    types::ReceiveWsMessage::ChannelStartTyping { channel_id, user_id } => {
                        typing_state
                            .entry(channel_id)
                            .or_default()
                            .insert(user_id);

                        set_typing_state(typing_state.clone());
                    },
                    types::ReceiveWsMessage::ChannelStopTyping { channel_id, user_id } => {
                        if let Some(channel) = typing_state.get_mut(&channel_id) {
                            channel.remove(&user_id);
                        }
                    },
                    types::ReceiveWsMessage::MessageUpdate { message_id, channel_id, data } => {
                        if let Some(channel) = message_state.get_mut(&channel_id) {
                            if let Some(message) = channel.get_mut(&message_id) {
                                message.update(data);
                                set_message_state(message_state.clone());
                            }
                        }
                    },
                    types::ReceiveWsMessage::MessageDelete { message_id, channel_id } => {
                        if let Some(channel) = message_state.get_mut(&channel_id) {
                            if channel.remove(&message_id).is_some() {
                                set_message_state(message_state.clone());
                            }
                        }
                    },
                    types::ReceiveWsMessage::MessageReact { message_id, channel_id, user_id, emoji_id } => {
                        if let Some(channel) = message_state.get_mut(&channel_id) {
                            if let Some(message) = channel.get_mut(&message_id) {
                                message.reactions
                                    .entry(emoji_id)
                                    .or_default()
                                    .insert(user_id);

                                set_message_state(message_state.clone());
                            }
                        }
                    },
                    types::ReceiveWsMessage::MessageUnreact { message_id, channel_id, user_id, emoji_id } => {
                        if let Some(channel) = message_state.get_mut(&channel_id) {
                            if let Some(message) = channel.get_mut(&message_id) {
                                if let Some(set) = message.reactions.get_mut(&emoji_id) {
                                    set.remove(&user_id);
                                    if set.is_empty() {
                                        message.reactions.remove(&emoji_id);
                                    }
                                };

                                set_message_state(message_state.clone());
                            }
                        }
                    },
                    types::ReceiveWsMessage::MessageRemoveReaction { message_id, channel_id, emoji_id } => {
                        if let Some(channel) = message_state.get_mut(&channel_id) {
                            if let Some(message) = channel.get_mut(&message_id) {
                                message.reactions.remove(&emoji_id);

                                set_message_state(message_state.clone());
                            }
                        }
                    },
                    types::ReceiveWsMessage::ServerMemberJoin { server_id, user_id } => {
                        if !user_state.contains_key(&user_id) {
                            let user = http.fetch_user(&user_id).await;
                            user_state.insert(user_id.clone(), user);
                            set_user_state(user_state.clone());
                        };

                        let server = &mut server_member_state[&server_id];

                        server.insert(user_id.clone(), types::Member::from_ids(server_id, user_id));
                        set_server_member_state(server_member_state.clone());
                    },
                    _ => {
                        log::info!("IGNORED EVENT: {:?}", event);
                    }
                },
                Err(error) => log::error!("{error:?}\n{payload}"),
            }
        }
    };

    join!(task1, task2);
}
