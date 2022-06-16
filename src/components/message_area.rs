use dioxus::prelude::*;
use futures::StreamExt;
use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MessageAreaProps {
    channel_id: types::ULID
}

pub fn MessageArea(cx: Scope<MessageAreaProps>) -> Element {
    let message = use_state(&cx, String::new);

    let send_message = use_coroutine::<String, _, _>(&cx, move |mut rx| {
        let http = cx.consume_context::<HTTPClient>().unwrap();
        let channel_id = cx.props.channel_id.clone();

        async move {
            while let Some(content) = rx.next().await {
                http.send_message(
                    &channel_id,
                    MessageBuilder::new()
                        .content(content)
                        .build(),
                )
                .await;
            }
        }
    });

    rsx!(cx, div {
        style: "height: 48px; background-color: blue; display: flex; flex-direction: row",
        input {
            style: "width: 90%",
            oninput: move |evt| {
                message.set(evt.value.clone());
            }
        },
        button {
            style: "width: 10%",
            onclick: move |_| {
                send_message.send(message.get().clone());
            },
            "Send"
        }
    })
}
