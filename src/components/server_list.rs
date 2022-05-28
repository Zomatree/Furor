use dioxus::prelude::*;
use crate::prelude::*;


pub fn ServerList(cx: Scope) -> Element {
    let server_state = use_read(&cx, SERVERS);

    rsx!(cx, div {
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
    })
}
