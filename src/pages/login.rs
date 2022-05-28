use dioxus::prelude::*;
use crate::{types, pages::MainApp};

#[derive(Props, PartialEq)]
pub struct LoginProps {
    pub revolt_config: types::RevoltConfig,
}

pub fn Login(cx: Scope<LoginProps>) -> Element {
    let token = use_state(&cx, String::new);
    let channel = use_state(&cx, String::new);
    let server = use_state(&cx, String::new);
    let button_clicked = use_state(&cx, || false);
    let token_type = use_state(&cx, || String::from("user"));

    cx.render(match button_clicked.get() {
        true => {
            rsx! {
                MainApp {
                    token: match token_type.as_str() {
                        "user" => types::Token::User(token.get().clone()),
                        "bot" => types::Token::Bot(token.get().clone()),
                        _ => unreachable!()
                    },
                    channel: types::ULID(channel.get().clone()),
                    server: types::ULID(server.get().clone()),
                    revolt_config: cx.props.revolt_config.clone()
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
                            token.set(evt.value.clone());
                        }
                    },
                    input {
                        style: "width: 30%; height: 48px; margin-top: 12px",
                        placeholder: "Enter channel ID",
                        oninput: |evt| {
                            channel.set(evt.value.clone());
                        }
                    },
                    input {
                        style: "width: 30%; height: 48px; margin-top: 12px",
                        placeholder: "Enter server ID",
                        oninput: |evt| {
                            server.set(evt.value.clone());
                        }
                    },
                    select {
                        style: "width: 30%; height: 48px; margin-top: 12px",
                        name: "token_type",
                        onchange: |evt| {
                            token_type.set(evt.value.clone());
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
                            button_clicked.set(true);
                        },
                        "Done",
                    }
                }
            }
        }
    })
}
