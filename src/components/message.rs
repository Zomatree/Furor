use dioxus::prelude::*;
use crate::prelude::*;


#[derive(Props, PartialEq)]
pub struct MessageProps {
    pub message_id: types::ULID,
    pub channel_id: types::ULID
}

pub fn Message(cx: Scope<MessageProps>) -> Element {
    let message_cache = use_read(&cx, MESSAGES);
    let message = message_cache
        .get(&cx.props.channel_id)?
        .get(&cx.props.message_id)?;

    let types::Message { content, author, attachments, channel, masquerade, replies, .. } = message;

    let user_cache = use_read(&cx, USERS);
    let user = user_cache.get(author).unwrap();
    let (username, avatar) = get_username_avatar(&cx, user, masquerade, channel);
    let content = content.clone().unwrap_or_default();

    cx.render(rsx! {
        div {
            replies
                .iter()
                .map(|reply| {
                    rsx! {
                        div {
                            key: "{reply}",
                            components::Reply {
                                message_id: reply.clone(),
                                channel_id: cx.props.channel_id.clone(),
                                message_mentions: replies.clone()
                            }
                        }
                    }
                })
            div {
                style: "display: flex; flex-direction: row",
                img {
                    src: "{avatar}",
                    width: "44",
                    height: "44"
                },
                h3 { "{username}" },
                {user.bot.is_some().then(|| rsx! {
                    span {
                        "[BOT]"
                    }
                })}
            },
            p { "{content}" }
            attachments.iter().enumerate().map(|(i, attachment)| {
                let url = attachment.url();

                rsx! {
                    div {
                        key: "{i}",
                        img {
                            src: "{url}",
                        }
                    }
                }
            })

        }
    })
}
