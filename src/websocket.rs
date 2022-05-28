use dioxus::prelude::*;
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
    mut user_state: UserCache,
    set_user_state: FermiSetter<UserCache>,
    mut server_state: ServerCache,
    set_server_state: FermiSetter<ServerCache>,
    mut channel_state: ChannelCache,
    set_channel_state: FermiSetter<ChannelCache>,
    mut server_member_state: ServerMemberCache,
    set_server_member_state: FermiSetter<ServerMemberCache>,
    mut message_state: MessageCache,
    set_message_state: FermiSetter<MessageCache>,
    mut typing_state: TypingCache,
    set_typing_state: FermiSetter<TypingCache>,
    ready: UseState<bool>,
) {
    let (_, ws) = WsMeta::connect(http.revolt_config.ws.clone(), None)
        .await
        .unwrap();

    let ws = Rc::new(RwLock::new(ws));

    let bg_ws = ws.clone();

    let (ready_tx, ready_rx) = oneshot::channel();

    join!(async move {
        ready_rx.await.unwrap();

        loop {
            bg_ws.write().await.send(WsMessage::Text(serde_json::to_string(&types::SendWsMessage::Ping { data: 0 }).unwrap())).await.unwrap();
            sleep(Duration::from_secs(30)).await;
        }
    },
    async move {
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
            log::debug!("EVENT RECEIVED {payload}");

            match serde_json::from_str::<types::ReceiveWsMessage>(&payload) {
                Ok(event) => match event {
                    types::ReceiveWsMessage::Authenticated => {
                        ready_tx.take().unwrap().send(()).unwrap();
                    }
                    types::ReceiveWsMessage::Ready {
                        users,
                        servers,
                        channels,
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
                            channel_state.insert(channel.id(), channel);
                        }

                        set_channel_state(channel_state.clone());

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

                        ready.set(true);
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
                    }
                    _ => {
                        log::info!("IGNORED EVENT: {event:?}");
                    }
                },
                Err(error) => log::error!("{error:?}\n{payload}"),
            }
        }
    });
}
