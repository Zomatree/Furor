use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReplyProps {
    pub message_id: types::ULID,
    pub channel_id: types::ULID,
    pub message_mentions: Vec<types::ULID>,
}

pub fn Reply(cx: Scope<ReplyProps>) -> Element {
    let http = cx.consume_context::<HTTPClient>().unwrap();

    let message_cache = use_read(&cx, MESSAGES);
    let set_message_cache = use_set(&cx, MESSAGES);

    let user_cache = use_read(&cx, USERS);

    let reply = use_state(&cx, || None::<types::Message>);

    cx.use_hook(|_| cx.spawn({
        let reply = reply.clone();
        let mut message_cache = message_cache.clone();
        let set_message_cache = set_message_cache.clone();
        let message_id = cx.props.message_id.clone();
        let channel_id = cx.props.channel_id.clone();
        let http = http.clone();

        async move {
            let channel = message_cache.entry(channel_id.clone()).or_default();

            let message = match channel.get(&message_id) {
                Some(message) => message.clone(),
                None => {
                    let message = http.fetch_message(&channel_id, &message_id).await;
                    channel.insert(message_id.clone(), message.clone());
                    message
                }
            };

            set_message_cache(message_cache);
            reply.set(Some(message))
        }
    }));

    cx.render(match reply.get() {
        Some(message) => {
            let message_id = &message.id;
            let (username, avatar) = get_username_avatar(&cx, user_cache.get(&message.author).unwrap(), &message.masquerade, &cx.props.channel_id);
            let content = message.content.clone().unwrap_or_default();

            let username = if cx.props.message_mentions.contains(&message.author) {
                format!("@{username}")
            } else {
                username
            };

            rsx! {
                div {
                    key: "{message_id}",
                    style: "display: flex; flex-direction: row",
                    img {
                        src: "{avatar}",
                        width: "14",
                        height: "14"
                    },
                    span {
                        "{username} "
                    },
                    span {
                        style: "font-size: 14px",
                        "{content}"
                    }
                }
            }
        },

        None => {
            rsx! {
                span { "Loading" }
            }
        }
    })
}
