use crate::prelude::*;

pub fn DirectMessageList(cx: Scope) -> Element {
    rsx!(cx, div {
        style: "width: 232px; height: 100%",
        h1 {
            "Direct Messages"
        },
        button {
            "Home"
        },
        button {
            "Friends"
        },
        button {
            "Saved Notes"
        },
        h3 {
            "Conversations"
        }
    })
}
