use dioxus::{prelude::*, core::to_owned};
use crate::{prelude::*, websocket::websocket};

#[derive(Props, PartialEq)]
pub struct AppProps {
    pub revolt_config: types::RevoltConfig,
    pub token: types::Token,
    pub channel: types::ULID,
    pub server: types::ULID
}

pub fn MainApp(cx: Scope<AppProps>) -> Element {
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

    cx.render(match ready.get() {
        true => rsx! {
            div {
                div {
                    style: "width: 100%; height: 100%; display: flex; flex-direction: row",
                    components::ServerList {},
                    div {
                        style: "display: flex; flex-direction: row; flex-grow: 1",
                        components::ChannelList {
                            server_id: active_server.get()
                        },
                        div {
                            style: "display: flex; flex-direction: column; width: 100%",
                            components::Channel {
                                channel_id: active_channel.get(),
                                server_id: active_server.get()
                            },
                            components::MessageArea {
                                channel_id: active_channel.get()
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
