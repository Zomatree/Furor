
use crate::prelude::*;

pub fn SavedMessages(cx: Scope) -> Element {
    redirect_to_login(&cx);

    let saved_messages = use_read(&cx, SAVED_MESSAGES).as_ref().unwrap();

    let router = use_router(&cx);

    router.push_route(&format!("/channel/{}", saved_messages.id), None, None);

    cx.render(rsx! {
        components::Loading {}
    })
}
