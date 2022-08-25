
use crate::prelude::*;

pub fn Channel(cx: Scope) -> Element {
    redirect_to_login(&cx);

    let route = use_route(&cx);

    let server_id = route.parse_segment::<types::ULID>("server_id").unwrap().unwrap();
    let channel_id = route.parse_segment::<types::ULID>("channel_id").unwrap().unwrap();

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: row",
        components::ServerList {},
        div {
            style: "display: flex; flex-direction: row; flex-grow: 1",
            div {
                style: "display: flex; flex-direction: column; flex-grow: 1",
                components::ServerHeader {
                    server_id: server_id.clone(),
                }
                components::ChannelList {
                    server_id: server_id.clone()
                },
            },
            components::Channel {
                channel_id: channel_id
            },
            // components::MemberList {
            //     channel_id: channel_id,
            //     server_id: server_id
            // }
        }
    })
}
