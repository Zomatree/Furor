use crate::prelude::*;

pub fn Home(cx: Scope) -> Element {
    redirect_to_login(&cx);

    rsx!(cx, div {
        style: "width: 100%; height: 100%; display: flex; flex-direction: row",
        components::ServerList {},
        div {
            style: "display: flex; flex-direction: row; flex-grow: 1",
            components::DirectMessageList {},
            h1 {
                "Welcome to Revolt"
            }
        }
    })
}
