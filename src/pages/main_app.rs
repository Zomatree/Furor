use dioxus::prelude::*;
use dioxus::core::to_owned;
use futures::{SinkExt, StreamExt, join};
use std::{collections::HashMap, time::Duration};
use ws_stream_wasm::{WsMessage, WsMeta};
use crate::prelude::*;
use std::sync::Arc;
use async_std::{sync::Mutex, task::sleep};

#[derive(Props, PartialEq)]
pub struct AppProps {
    pub revolt_config: types::RevoltConfig,
    pub token: types::Token,
    pub channel: types::ULID,
    pub server: types::ULID
}

pub fn MainApp(cx: Scope<AppProps>) -> Element {
    let messages = use_state(&cx, Vec::<types::Message>::new);
    let message = use_state(&cx, String::new);
    let active_channel = use_state(&cx, || cx.props.channel.clone());
    let active_server = use_state(&cx, || cx.props.server.clone());
    let ready = use_state(&cx, || false);
    let currently_typing = use_state(&cx, HashMap::<types::ULID, (String, String)>::new);

    let user_state = use_read(&cx, USERS);
    let set_user_state = use_set(&cx, USERS);

    let server_state = use_read(&cx, SERVERS);
    let set_server_state = use_set(&cx, SERVERS);

    let channel_state = use_read(&cx, CHANNELS);
    let set_channel_state = use_set(&cx, CHANNELS);

    let server_members_state = use_read(&cx, SERVER_MEMBERS);
    let set_server_members_state = use_set(&cx, SERVER_MEMBERS);

    let message_state = use_read(&cx, MESSAGES);
    let set_message_state = use_set(&cx, MESSAGES);

    cx.use_hook(|_| {
        let http = HTTPClient::new(cx.props.token.clone(), API_URL, cx.props.revolt_config.clone());
        cx.provide_context(http);
    });

    let send_message = use_coroutine::<String, _, _>(&cx, move |mut rx| {
        let active_channel = active_channel.to_owned();
        let http = cx.consume_context::<HTTPClient>().unwrap();

        async move {
            while let Some(content) = rx.next().await {
                http.send_message(
                    active_channel.get(),
                    MessageBuilder::new()
                        .content(content)
                        .build(),
                )
                .await;
            }
        }
    });

    cx.use_hook(|_| cx.spawn({
        to_owned![
            ready,
            messages,
            active_channel,
            active_server,
            currently_typing,
            user_state,
            set_user_state,
            server_state,
            set_server_state,
            channel_state,
            set_channel_state,
            server_members_state,
            set_server_members_state,
            message_state,
            set_message_state
        ];

        let http = cx.consume_context::<HTTPClient>().unwrap();
        async move {
            let (_, ws) = WsMeta::connect(http.revolt_config.ws.clone(), None)
                .await
                .unwrap();

            let ws = Arc::new(Mutex::new(ws));

            let bg_ws = ws.clone();

            join!(async move {
                loop {
                    bg_ws.lock().await.send(WsMessage::Text(serde_json::to_string(&types::SendWsMessage::Ping { data: 0 }).unwrap())).await.unwrap();
                    sleep(Duration::from_secs(15)).await;
                }
            },
            async move {
                ws.lock().await.send(WsMessage::Text(
                    serde_json::to_string(&types::SendWsMessage::Authenticate {
                        token: http.token.inner().to_string(),
                    })
                    .unwrap(),
                ))
                .await
                .unwrap();

                while let Some(WsMessage::Text(payload)) = ws.lock().await.next().await {
                    log::debug!("EVENT RECEIVED {payload}");

                    match serde_json::from_str::<types::ReceiveWsMessage>(&payload) {
                        Ok(event) => match event {
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

                                    server_members_state.insert(server.id.clone(), current_server_members);
                                };

                                set_user_state(user_state.clone());
                                set_server_members_state(server_members_state.clone());

                                ready.set(true);
                            },
                            types::ReceiveWsMessage::Message { message } => {
                                if &message.channel == active_channel.get() {
                                    log::info!("Pushing message to current channel");
                                    messages.with_mut(|messages| messages.push(message.clone()))
                                };

                                message_state
                                    .entry(message.channel.clone())
                                    .or_default()
                                    .insert(message.id.clone(), message);

                                set_message_state(message_state.clone());
                            },
                            types::ReceiveWsMessage::ChannelStartTyping { channel_id, user_id } => {
                                if let Some(user) = user_state.get(&user_id) && &channel_id == active_channel.get() {
                                    let member = server_members_state
                                        .get(active_server.get())
                                        .unwrap()
                                        .get(&user_id)
                                        .unwrap();

                                    currently_typing.with_mut(|typing| {
                                        typing.insert(user_id, (
                                            member
                                                .nickname
                                                .clone()
                                                .unwrap_or_else(|| user.username.clone()),
                                            member
                                                .avatar
                                                .as_ref()
                                                .or(user.avatar.as_ref())
                                                .unwrap()
                                                .url(),
                                        ));
                                    });
                                }
                            },
                            types::ReceiveWsMessage::ChannelStopTyping { channel_id, user_id } => {
                                if &channel_id == active_channel.get() {
                                    currently_typing.with_mut(|state| {
                                        state.remove(&user_id);
                                    })
                                }
                            },
                            types::ReceiveWsMessage::MessageUpdate { message_id, channel_id, data } => {
                                if let Some(channel) = message_state.get_mut(&channel_id) {
                                    if let Some(message) = channel.get_mut(&message_id) {
                                        message.update(data)
                                    }
                                }
                            }
                            _ => {
                                log::info!("IGNORED EVENT: {event:?}")
                            }
                        },
                        Err(error) => log::error!("{error:?}\n{payload}"),
                    }
                }
            });
        }
    }));

    cx.render(match ready.get() {
        true => rsx! {
            div {
                div {
                    style: "width: 100%; height: 100%; display: flex; flex-direction: row",
                    div {
                        style: "display: flex; width: 56px; min-width: 56px; flex-direction: column; justify-content: flex-start; overflow-y: auto; align-items: center",
                        server_state.values().map(|server| {
                            let types::Server { icon, id, .. } = server;

                            let icon = icon.clone().unwrap().url();

                            rsx! {
                                img {
                                    key: "{id}",
                                    src: "{icon}",
                                    height: "42",
                                    width: "42",
                                    onclick: |_| {

                                    }
                                }
                            }
                        })
                    },
                    div {
                        style: "display: flex; flex-direction: row; flex-grow: 1",
                        div {
                            style: "display: flex; flex-direction: column",
                            server_state
                                .get(active_server.get())
                                .unwrap()
                                .channels
                                .iter()
                                .filter_map(|channel_id| channel_state.get(channel_id).cloned())
                                .map(|channel| {
                                    match channel {
                                        types::Channel::TextChannel { id, name, .. } => {
                                            rsx! {
                                                div {
                                                    key: "{id}",
                                                    style: "display: flex; flex-direction: row",
                                                    span {
                                                        "# ",
                                                        "{name}"
                                                    },
                                                }
                                            }
                                        },
                                        types::Channel::VoiceChannel { id, name, .. } => {
                                            rsx! {
                                                div {
                                                    key: "{id}",
                                                    style: "display: flex; flex-direction: row",
                                                    span {
                                                        "V ",
                                                        "{name}"
                                                    },
                                                }
                                            }
                                        },
                                        _ => unreachable!()
                                    }
                                })
                        }
                        div {
                            style: "display: flex; flex-direction: column; width: 100%",
                            div {
                                style: "background-color: grey; overflow-y: auto; flex-grow: 1",
                                messages.iter().map(|msg| {
                                    let message_id = &msg.id;

                                    rsx! {
                                        div {
                                            key: "{message_id}",
                                            components::Message {
                                                channel_id: &msg.channel,
                                                message_id: &msg.id,
                                            }
                                        }
                                    }
                                })
                            },
                            (!currently_typing.is_empty()).then(|| {
                                let mut avatars = vec![];
                                let mut names = vec![];

                                for (name, avatar) in currently_typing.values().cloned() {
                                    avatars.push(avatar);
                                    names.push(name);
                                };

                                let formatted_string = names.join(", ");

                                rsx! {
                                    div {
                                        style: "z-index: 2",
                                        avatars.iter().map(|url| {
                                            rsx! {
                                                img {
                                                    src: "{url}",
                                                    width: "16",
                                                    height: "16"
                                                }
                                            }
                                        }),
                                        "{formatted_string} are typing..."
                                    }
                                }
                            }),
                            div {
                                style: "height: 48px; background-color: blue; display: flex; flex-direction: row",
                                input {
                                    style: "width: 90%",
                                    oninput: move |evt| {
                                        message.set(evt.value.clone())

                                    }
                                },
                                button {
                                    style: "width: 10%",
                                    onclick: move |_| {
                                        send_message.send(message.get().clone())
                                    },
                                    "Send"
                                }
                            }
                        }
                    }
                }
            }
        },
        false => rsx! {
            h1 {
                "Loading..."
            }
        }
    })
}
