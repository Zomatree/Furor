use dioxus::{prelude::*, core::to_owned};
use futures::StreamExt;
use crate::{prelude::*, websocket::websocket};

#[derive(Props, PartialEq)]
pub struct AppProps {
    pub revolt_config: types::RevoltConfig,
    pub token: types::Token,
    pub channel: types::ULID,
    pub server: types::ULID
}

pub fn MainApp(cx: Scope<AppProps>) -> Element {
    let message = use_state(&cx, String::new);

    let active_channel = use_state(&cx, || cx.props.channel.clone());
    let active_server = use_state(&cx, || cx.props.server.clone());
    let ready = use_state(&cx, || false);

    let user_state = use_read(&cx, USERS);
    let set_user_state = use_set(&cx, USERS);

    let server_state = use_read(&cx, SERVERS);
    let set_server_state = use_set(&cx, SERVERS);

    let channel_state = use_read(&cx, CHANNELS);
    let set_channel_state = use_set(&cx, CHANNELS);

    let server_member_state = use_read(&cx, SERVER_MEMBERS);
    let set_server_member_state = use_set(&cx, SERVER_MEMBERS);

    let message_state = use_read(&cx, MESSAGES);
    let set_message_state = use_set(&cx, MESSAGES);

    let typing_state = use_read(&cx, TYPING);
    let set_typing_state = use_set(&cx, TYPING);

    cx.use_hook(|_| {
        to_owned![
            user_state,
            set_user_state,
            server_state,
            set_server_state,
            channel_state,
            set_channel_state,
            server_member_state,
            set_server_member_state,
            message_state,
            set_message_state,
            typing_state,
            set_typing_state,
            ready
        ];

        let http = HTTPClient::new(cx.props.token.clone(), API_URL, cx.props.revolt_config.clone());
        cx.provide_context(http.clone());

        cx.spawn(async move {
            websocket(
                http,
                user_state.clone(),
                set_user_state.clone(),
                server_state.clone(),
                set_server_state.clone(),
                channel_state.clone(),
                set_channel_state.clone(),
                server_member_state.clone(),
                set_server_member_state.clone(),
                message_state.clone(),
                set_message_state.clone(),
                typing_state.clone(),
                set_typing_state.clone(),
                ready.clone()
            ).await;
        })
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

    let mut messages = message_state
        .get(active_channel.get())
        .cloned()
        .unwrap_or_default()
        .values()
        .cloned()
        .collect::<Vec<_>>();

    messages.sort_by(|a, b| a.id.timestamp().cmp(&b.id.timestamp()));

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
                                messages.into_iter().map(|msg| {
                                    let message_id = msg.id.clone();

                                    rsx! {
                                        div {
                                            key: "{message_id}",
                                            components::Message {
                                                channel_id: msg.channel,
                                                message_id: message_id.clone(),
                                            }
                                        }
                                    }
                                })
                            },
                            typing_state.get(active_channel).map(|currently_typing| {
                                let mut avatars = vec![];
                                let mut names = vec![];

                                for user_id in currently_typing {
                                    let user = user_state.get(user_id).unwrap();
                                    let (username, avatar) = get_username_avatar(&cx, user, &None, active_channel.get());

                                    names.push(username);
                                    avatars.push(avatar);
                                };

                                let formatted_string = names.join(", ");

                                rsx! {
                                    div {
                                        style: "z-index: 2",
                                        avatars.iter().map(|url| {
                                            rsx! {
                                                img {
                                                    key: "{url}",
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
