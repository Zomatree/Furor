#![feature(explicit_generic_args_with_impl_trait)]
#![feature(async_closure)]

pub mod types;
pub mod http;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus::fermi::prelude::{use_read, Atom};
use futures::{StreamExt, channel::mpsc, Future, SinkExt};
use ws_stream_wasm::{WsMeta, WsMessage};

const URL: &str = "revolt.chat";
const AUTUMN_URL: &str = "autumn.revolt.chat";
const TOKEN: &str = "";
const CHANNEL: &str = "01FD59DX5JF56V5S0B89WNF51H";

static HTTP: Atom<http::HTTPClient> = |_| http::HTTPClient::new(types::Token::User(TOKEN.to_string()), format!("api.{URL}"));

#[derive(Default, Clone)]
pub struct State {
    pub users: HashMap<types::ULID, types::User>,
    pub servers: HashMap<types::ULID, types::Server>,
    pub channels: HashMap<types::ULID, types::Channel>,
    pub server_members: HashMap<types::ULID, HashMap<types::ULID, types::Member>>
}

fn app(cx: Scope) -> Element {
    let (messages, set_messages) = use_state(&cx, Vec::<types::Message>::new);
    let (message, set_message) = use_state(&cx, String::new);
    let (state, set_state) = use_state(&cx, State::default);
    let http = use_read(&cx, HTTP).clone();

    cx.use_hook(|_| {
        let sender = use_async_channel::<String, _>(&cx, move |content| {
            let http = http.clone();

            async move {
                http.send_message(types::ULID(CHANNEL.to_string()), types::SendMessage::with_content(content))
                    .await;
            }
        });

        cx.provide_context(sender);
    });

    use_future(&cx, move || {
        let http = use_read(&cx, HTTP).clone();
        to_owned![set_messages, set_state];

        async move {
            let (_, mut ws) = WsMeta::connect(format!("wss://ws.{URL}"), None).await.unwrap();


            ws.send(WsMessage::Text(serde_json::to_string(&types::SendWsMessage::Authenticate { token: TOKEN.to_string() }).unwrap())).await.unwrap();

            while let Some(WsMessage::Text(payload)) = ws.next().await {
                match serde_json::from_str::<types::ReceiveWsMessage>(&payload) {
                    Ok(event) => {
                        match event {
                            types::ReceiveWsMessage::Ready { users, servers, channels } => {
                                set_state.with_mut(|state| {
                                    for user in users {
                                        state.users.insert(user.id.clone(), user);
                                    };

                                    for server in servers.clone() {
                                        state.servers.insert(server.id.clone(), server);
                                    };

                                    for channel in channels {
                                        state.channels.insert(channel.id(), channel);
                                    };
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
                            },
                            types::ReceiveWsMessage::Message { message } => {
                                if message.channel.0.as_str() == CHANNEL {
                                    set_messages.with_mut(|messages| messages.push(message))
                                }
                            },
                            _ => {
                                log::info!("IGNORED EVENT: {event:?}")
                            }
                        }
                    },
                    Err(error) => log::error!("{error:?}")
                }
            }
        }
    });

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: column",
        div {
            style: "background-color: grey; overflow-y: scroll; flex-grow: 1",
            messages.iter().map(|msg| {
                let types::Message { content, id, author, attachments, channel, masquerade, .. } = msg;

                let user = author.to_user(state).unwrap();

                let (username, avatar) = match masquerade {
                    Some(mask) => {
                        (mask.name.clone().unwrap_or_else(|| user.username.clone()), mask.avatar.clone().unwrap_or_else(|| user.avatar.clone().unwrap().url(AUTUMN_URL.to_string())))
                    },
                    None => {
                        let channel = channel.to_channel(state).unwrap();

                        match channel.server() {
                            Some(server_id) => {
                                let member = author.to_member(state, &server_id).unwrap();
                                (member.nickname.clone().unwrap_or_else(|| user.username.clone()), member.avatar.clone().unwrap_or_else(|| user.avatar.clone().unwrap()).url(AUTUMN_URL.to_string()))
                            },
                            None => {
                                (user.username.clone(), user.avatar.clone().unwrap().url(AUTUMN_URL.to_string()))
                            }
                        }
                    }
                };

                rsx! {
                    div {
                        key: "{id}",
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
                        (!attachments.is_empty()).then(|| {
                            rsx! {
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

pub fn use_async_channel<'a, T: 'static, F: Future<Output = ()> + 'static>(cx: &'a ScopeState, callback: impl Fn(T) -> F + 'static) -> mpsc::UnboundedSender<T> {
    let (tx, mut rx) = mpsc::unbounded::<T>();

    cx.spawn(async move {
        while let Some(value) = rx.next().await {
            callback(value).await;
        }
    });

    tx
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(app);
}
