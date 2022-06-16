use dioxus::prelude::*;
use crate::prelude::*;

pub fn ServerList(cx: Scope) -> Element {
    let set_current_server = use_set(&cx, CURRENT_SERVER);
    let set_current_channel = use_set(&cx, CURRENT_CHANNEL);
    let server_state = use_read(&cx, SERVERS);
    let channel_state = use_read(&cx, CHANNELS);

    rsx!(cx, div {
        style: "display: flex; width: 56px; min-width: 56px; flex-direction: column; justify-content: flex-start; overflow-y: auto; align-items: center",
        server_state.values().map(|server| {
            let types::Server { icon, id, .. } = server.clone();

            let icon = icon.unwrap().url();
            let key = id.clone();

            rsx! {
                img {
                    key: "{key}",
                    src: "{icon}",
                    height: "42",
                    width: "42",
                    onclick: move |_| {
                        let id = id.clone();

                        set_current_server(Some(id.clone()));

                        let channel = get_last_channel(&id).unwrap_or_else(|| {
                            server_state[&id].channels
                                .iter()
                                .map(|id| &channel_state[id])
                                .find(|channel| matches!(channel, types::Channel::TextChannel { .. }))
                                .unwrap()
                                .id()
                        });

                        set_current_channel(Some(channel));
                    }
                }
            }
        })
    })
}
