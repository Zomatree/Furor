
use crate::prelude::*;

pub fn Channel(cx: Scope) -> Element {
    redirect_to_login(&cx);

    let route = use_route(&cx);
    let channel_state = use_read(&cx, CHANNELS);

    let server_id = route.parse_segment::<types::ULID>("server_id").unwrap().unwrap();
    let channel_id = route.parse_segment::<types::ULID>("channel_id").unwrap().unwrap();
    log::info!("{server_id:?} {channel_id:?}");

    let channel = &channel_state[&channel_id];

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: row",
        components::ServerList {},
        div {
            style: "display: flex; flex-direction: row; flex-grow: 1",
            components::ChannelList {
                server_id: server_id.clone()
            },
            div {
                style: "display: flex; flex-direction: column; width: 100%",
                div {
                    style: "height: 48px; width: 100%",
                    channel.name()
                }
                components::Channel {
                    channel_id: channel_id.clone(),
                    server_id: server_id.clone()
                },
                components::Typing {
                    channel_id: channel_id.clone(),

                }
                components::MessageArea {
                    channel_id: channel_id
                }
            }
        }
    })
}
