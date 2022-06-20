use crate::prelude::*;

pub fn ServerList(cx: Scope) -> Element {
    let set_current_server = use_set(&cx, CURRENT_SERVER);
    let set_current_channel = use_set(&cx, CURRENT_CHANNEL);
    let server_state = use_read(&cx, SERVERS);
    let channel_state = use_read(&cx, CHANNELS);
    let router = use_router(&cx);

    rsx!(cx, div {
        style: "display: flex; width: 56px; min-width: 56px; flex-direction: column; justify-content: flex-start; overflow-y: auto; align-items: center",
        server_state.values().map(|server| {
            let types::Server { icon, id, .. } = server.clone();

            let icon = icon.unwrap().url();
            let key = id.clone();

            rsx! {
                button {
                    key: "{key}",
                    onclick: move |_| {
                        set_current_server(Some(id.clone()));

                        let channel = get_last_channel(&id).unwrap_or_else(|| {
                            server_state[&id].channels
                                .iter()
                                .map(|id| &channel_state[id])
                                .find(|channel| matches!(channel, types::Channel::TextChannel { .. }))
                                .unwrap()
                                .id()
                        });

                        set_current_channel(Some(channel.clone()));
                        router.push_route(&format!("/server/{id}/channel/{channel}"), None, None);
                    },
                    img {
                        src: "{icon}",
                        height: "42",
                        width: "42"
                    }
                }
            }
        })
    })
}
