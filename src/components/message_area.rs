use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MessageAreaProps {
    channel_id: types::ULID
}

pub fn MessageArea(cx: Scope<MessageAreaProps>) -> Element {
    let message = use_state(&cx, String::new);
    let http = use_read(&cx, HTTP).clone().unwrap();

    rsx!(cx, div {
        style: "min-height: 48px; background-color: blue; display: flex; flex-direction: row",
        input {
            style: "width: 90%",
            oninput: move |evt| {
                message.set(evt.value.clone());
            }
        },
        button {
            style: "width: 10%",
            onclick: move |_| {
                let channel_id = cx.props.channel_id.clone();
                let http = http.clone();
                let content = message.get().clone();

                cx.spawn(async move {
                    http.send_message(
                        &channel_id, MessageBuilder::new().content(content).build(),
                    ).await;
                })
            },
            "Send"
        }
    })
}
