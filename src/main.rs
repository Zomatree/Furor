#![feature(explicit_generic_args_with_impl_trait)]
#![feature(async_closure)]
#![allow(non_snake_case)]

pub mod http;
pub mod types;

use dioxus::prelude::*;
use futures::{channel::mpsc, Future, SinkExt, StreamExt};
use std::collections::HashMap;
use ws_stream_wasm::{WsMessage, WsMeta};

const URL: &str = "revolt.chat";
const AUTUMN_URL: &str = "autumn.revolt.chat";

#[derive(Default, Clone)]
pub struct State {
    pub users: HashMap<types::ULID, types::User>,
    pub servers: HashMap<types::ULID, types::Server>,
    pub channels: HashMap<types::ULID, types::Channel>,
    pub server_members: HashMap<types::ULID, HashMap<types::ULID, types::Member>>,
    pub message_cache: HashMap<types::ULID, HashMap<types::ULID, types::Message>>,
}

impl State {
    pub fn get_or_insert_message_cache(
        &mut self,
        channel_id: types::ULID,
    ) -> &mut HashMap<types::ULID, types::Message> {
        self.message_cache
            .entry(channel_id)
            .or_insert_with(HashMap::new)
    }

    pub fn get_message_from_cache(
        &self,
        channel_id: &types::ULID,
        message_id: &types::ULID,
    ) -> Option<&types::Message> {
        self.message_cache
            .get(channel_id)
            .and_then(|cache| cache.get(message_id))
    }
}

#[derive(Props, PartialEq)]
pub struct AppProps {
    pub token: types::Token,
    pub channel: String,
}

fn MainApp(cx: Scope<AppProps>) -> Element {
    let (messages, set_messages) = use_state(&cx, Vec::<types::Message>::new);
    let (message, set_message) = use_state(&cx, String::new);
    let (state, set_state) = use_state(&cx, State::default);

    cx.use_hook(|_| {
        let http = http::HTTPClient::new(cx.props.token.clone(), format!("api.{URL}"));
        cx.provide_context(http);
    });

    cx.use_hook(|_| {
        let http = cx.consume_context::<http::HTTPClient>().unwrap();
        let channel = cx.props.channel.clone();

        let sender = use_async_channel::<String, _>(&cx, move |content| {
            let http = (*http).to_owned();
            let channel = channel.to_owned();

            async move {
                http.send_message(
                    types::ULID(channel),
                    types::SendMessage::with_content(content),
                )
                .await;
            }
        });

        cx.provide_context(sender);
    });

    use_future(&cx, move || {
        to_owned![set_messages, set_state];
        let http = cx.consume_context::<http::HTTPClient>().unwrap();
        let token = cx.props.token.clone();
        let channel = cx.props.channel.clone();

        async move {
            let (_, mut ws) = WsMeta::connect(format!("wss://ws.{URL}"), None)
                .await
                .unwrap();

            ws.send(WsMessage::Text(
                serde_json::to_string(&types::SendWsMessage::Authenticate {
                    token: token.inner(),
                })
                .unwrap(),
            ))
            .await
            .unwrap();

            while let Some(WsMessage::Text(payload)) = ws.next().await {
                match serde_json::from_str::<types::ReceiveWsMessage>(&payload) {
                    Ok(event) => match event {
                        types::ReceiveWsMessage::Ready {
                            users,
                            servers,
                            channels,
                        } => {
                            set_state.with_mut(|state| {
                                for user in users {
                                    state.users.insert(user.id.clone(), user);
                                }

                                for server in servers.clone() {
                                    state.servers.insert(server.id.clone(), server);
                                }

                                for channel in channels {
                                    state.channels.insert(channel.id(), channel);
                                }
                            });

                            for server in servers {
                                let members = http.fetch_server_members(server.id.clone()).await;

                                set_state.with_mut(move |state| {
                                    let types::ServerMembers { users, members } = members;

                                    for user in users {
                                        state.users.insert(user.id.clone(), user);
                                    }

                                    let mut member_hashmap = HashMap::new();

                                    for member in members {
                                        member_hashmap.insert(member.id.user.clone(), member);
                                    }

                                    state.server_members.insert(server.id, member_hashmap);
                                })
                            }
                        }
                        types::ReceiveWsMessage::Message { message } => {
                            if message.channel.0 == channel {
                                set_messages.with_mut(|messages| messages.push(message.clone()))
                            };

                            set_state.with_mut(move |state| {
                                let message_cache =
                                    state.get_or_insert_message_cache(message.channel.clone());
                                message_cache.insert(message.id.clone(), message);
                            })
                        }
                        _ => {
                            log::info!("IGNORED EVENT: {event:?}")
                        }
                    },
                    Err(error) => log::error!("{error:?}"),
                }
            }
        }
    });

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: column",
        div {
            style: "background-color: grey; overflow-y: auto; flex-grow: 1",
            messages.iter().map(|msg| {
                let types::Message { content, id, author, attachments, channel, masquerade, replies, .. } = msg;

                let user = author.to_user(state).unwrap();
                let (username, avatar) = get_username_avatar(state, user, masquerade, channel);

                rsx! {
                    div {
                        key: "{id}",
                        replies.iter().map(|reply| {
                            let types::Message { content, author, masquerade, ..} = state.get_message_from_cache(channel, reply).unwrap();

                            let reply_user = author.to_user(state).unwrap();

                            let (username, avatar) = get_username_avatar(state, reply_user, masquerade, channel);

                            rsx! {
                                div {
                                    style: "display: flex; flex-direction: row",
                                    img {
                                        src: "{avatar}",
                                        width: "14",
                                        height: "14"
                                    },
                                    span {
                                        "{username}"
                                    },
                                    span {
                                        style: "font-size: 14px",
                                        "{content}"
                                    }
                                }
                            }
                        }),
                        div {
                            style: "display: flex; flex-direction: row",
                            img {
                                src: "{avatar}",
                                width: "44",
                                height: "44"
                            },
                            h3 { "{username}" }
                        },
                        p { "{content}" }
                        attachments.iter().map(|attachment| {
                            let url = attachment.url(AUTUMN_URL.to_string());

                            rsx! {
                                div {
                                    img {
                                        src: "{url}",
                                    }
                                }
                            }
                        })

                    }
                }
            })
        },
        div {
            style: "height: 48px; background-color: blue; display: flex; flex-direction: row",
            input {
                style: "width: 90%",
                oninput: move |evt| {
                    set_message(evt.value.clone())

                }
            },
            button {
                style: "width: 10%",
                onclick: move |_| {
                    let sender = cx.consume_context::<mpsc::UnboundedSender<String>>().unwrap();
                    sender.unbounded_send(message.clone()).unwrap();
                },
                "Send"
            }
        }
    })
}

pub fn use_async_channel<'a, T: 'static, F: Future<Output = ()> + 'static>(
    cx: &'a ScopeState,
    callback: impl Fn(T) -> F + 'static,
) -> mpsc::UnboundedSender<T> {
    let (tx, mut rx) = mpsc::unbounded::<T>();

    cx.spawn(async move {
        while let Some(value) = rx.next().await {
            callback(value).await;
        }
    });

    tx
}

pub fn app(cx: Scope) -> Element {
    let (token, set_token) = use_state(&cx, String::new);
    let (channel, set_channel) = use_state(&cx, String::new);
    let (button_clicked, set_button_clicked) = use_state(&cx, || false);
    let (token_type, set_token_type) = use_state(&cx, || String::from("user"));

    cx.render(match button_clicked {
        true => {
            rsx! {
                MainApp {
                    token: match token_type.as_str() {
                        "user" => types::Token::User(token.clone()),
                        "bot" => types::Token::Bot(token.clone()),
                        _ => unreachable!()
                    },
                    channel: channel.clone()
                }
            }
        },
        false => {
            rsx! {
                div {
                    style: "height: 100%; width: 100%; display: flex; flex-direction: column; justify-content: center; align-items: center",
                    h1 { "Client" },
                    input {
                        style: "width: 30%; height: 48px",
                        placeholder: "Enter token",
                        oninput: |evt| {
                            set_token(evt.value.clone());
                        }
                    },
                    input {
                        style: "width: 30%; height: 48px; margin-top: 12px",
                        placeholder: "Enter channel ID",
                        oninput: |evt| {
                            set_channel(evt.value.clone());
                        }
                    },
                    select {
                        style: "width: 30%; height: 48px; margin-top: 12px",
                        name: "token_type",
                        onchange: |evt| {
                            set_token_type(evt.value.clone())
                        },
                        option {
                            value: "user",
                            "User",
                        },
                        option {
                            value: "bot",
                            "Bot"
                        },
                    }
                    button {
                        style: "width: 8%; height: 48px; margin-top: 12px",
                        onclick: |_| {
                            set_button_clicked(true)
                        },
                        "Done",
                    }
                }
            }
        }
    })
}

pub fn get_username_avatar(
    state: &State,
    user: &types::User,
    masquerade: &Option<types::Masquerade>,
    channel_id: &types::ULID,
) -> (String, String) {
    match masquerade {
        Some(mask) => (
            mask.name.clone().unwrap_or_else(|| user.username.clone()),
            mask.avatar
                .clone()
                .unwrap_or_else(|| user.avatar.clone().unwrap().url(AUTUMN_URL.to_string())),
        ),
        None => {
            let channel = channel_id.to_channel(state).unwrap();

            match channel.server() {
                Some(server_id) => {
                    let member = user.id.to_member(state, &server_id).unwrap();
                    (
                        member
                            .nickname
                            .clone()
                            .unwrap_or_else(|| user.username.clone()),
                        member
                            .avatar
                            .clone()
                            .unwrap_or_else(|| user.avatar.clone().unwrap())
                            .url(AUTUMN_URL.to_string()),
                    )
                }
                None => (
                    user.username.clone(),
                    user.avatar.clone().unwrap().url(AUTUMN_URL.to_string()),
                ),
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}
