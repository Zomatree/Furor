use crate::prelude::*;


pub fn DmChannel(cx: Scope) -> Element {
    redirect_to_login(&cx);

    let route = use_route(&cx);

    let channel_id = route.parse_segment::<types::ULID>("channel_id").unwrap().unwrap();

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: row",
        components::ServerList {},
        div {
            style: "display: flex; flex-direction: row; flex-grow: 1",
            components::DirectMessageList {},
            components::Channel {
                channel_id: channel_id,
                name: String::from("temp name")
            }
        }
    })
}
