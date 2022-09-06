use im_rc::HashMap;

use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelProps<'a> {
    channel_id: &'a types::ULID
}

pub fn ChannelMessages<'a>(cx: Scope<'a, ChannelProps<'a>>) -> Element<'a> {
    let message_state = use_read(&cx, MESSAGES);
    let default = cx.use_hook(HashMap::new);

    let mut messages = message_state
        .get(cx.props.channel_id)
        .unwrap_or(default)
        .values()
        .collect::<Vec<_>>();

    messages.sort_by(|&a, &b| a.id.timestamp().cmp(&b.id.timestamp()));

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%; flex-grow: 1; background-color: grey; overflow-y: scroll",
            messages.iter().map(|message| {
                rsx! {
                    div {
                        key: "{message.id}",
                        components::Message {
                            channel_id: &message.channel,
                            message_id: &message.id,
                        }
                    }
                }
            })
        }
    })
}
