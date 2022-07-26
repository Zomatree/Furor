use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MessageAreaProps {
    channel_id: types::ULID
}

pub fn MessageArea(cx: Scope<MessageAreaProps>) -> Element {
    let message_builder_state = use_read(&cx, MESSAGE_BUILDERS);
    let set_message_builders = use_set(&cx, MESSAGE_BUILDERS);

    let message_state = use_read(&cx, MESSAGES);
    let channel_state = use_read(&cx, CHANNELS);
    let member_state = use_read(&cx, SERVER_MEMBERS);
    let revolt_config = use_read(&cx, REVOLT_CONFIG).as_ref().unwrap();
    let users = use_read(&cx, USERS);

    let http = use_read(&cx, HTTP).as_ref().unwrap();

    let channel_messages = message_state.get(&cx.props.channel_id).cloned().unwrap_or_default();

    let message_builder = match message_builder_state.get(&cx.props.channel_id) {
        Some(message_builder) => message_builder.clone(),
        None => {
            let message_builder = utils::MessageBuilder::new();
            let mut message_builders = message_builder_state.clone();
            message_builders.insert(cx.props.channel_id.clone(), message_builder);
            message_builders.get(&cx.props.channel_id).unwrap().clone()
        }
    };

    let content_message_builder = message_builder.clone();
    let attachments_message_builder = message_builder.clone();

    let replies_set_message_builders = set_message_builders.clone();
    let attachment_set_message_builder = set_message_builders.clone();

    rsx!(cx, div {
        style: "display: flex; flex-direction: column",
        div {
            style: "display: flex; flex-direction: column",
            div {
                style: "display: flex; flex-direction: row",
                message_builder.attachments.iter().flatten().map(|attachment| {
                    let name = attachment.name();

                    rsx! {
                        "{name}"
                    }
                })
            },
            message_builder.replies.iter().flatten().enumerate().map(|(i, reply)| {
                let message_builder1 = message_builder.clone();
                let message_builder2 = message_builder.clone();

                let mention_set_message_builders = replies_set_message_builders.clone();
                let remove_set_message_builders = set_message_builders.clone();

                let message = channel_messages.get(&reply.id).unwrap();
                let user = users.get(&message.author).unwrap();

                let (username, avatar) = utils::get_username_avatar(channel_state, member_state, revolt_config, user, &message.masquerade, Some(&cx.props.channel_id));
                rsx! {
                    div {
                        "Replying to "
                        components::Icon {
                            src: avatar
                        },
                        "{username}",
                        message.content.as_ref().map(|content| rsx! { [content.as_str()] }),

                        button {
                            onclick: move |_| {
                                let message_builder = message_builder1.clone();
                                let mut replies = message_builder.replies.as_ref().unwrap().clone();
                                let mut reply = replies.remove(i);

                                reply.mention = !reply.mention;
                                replies.insert(i, reply);

                                let mut message_builders = message_builder_state.clone();
                                message_builders.insert(cx.props.channel_id.clone(), message_builder.replies(replies));
                                mention_set_message_builders(message_builders);
                            },
                            if reply.mention {
                                ["@ on"]
                            } else {
                                ["@ off"]
                            },
                        },
                        button {
                            onclick: move |_| {
                                let message_builder = message_builder2.clone();
                                let mut replies = message_builder.replies.as_ref().unwrap().clone();

                                replies.remove(i);

                                let mut message_builders = message_builder_state.clone();
                                message_builders.insert(cx.props.channel_id.clone(), message_builder.replies(replies));
                                remove_set_message_builders(message_builders);
                            },
                            "X"
                        }
                    }
                }
            }),
        }
        div {
            style: "min-height: 48px; display: flex; flex-direction: row",
            button {
                style: "with: 10%",
                onclick: move |_| {
                    let attachments_message_builder = attachments_message_builder.clone();
                    let attachment_set_message_builder = attachment_set_message_builder.clone();
                    let message_builder_state = message_builder_state.clone();
                    let id = cx.props.channel_id.clone();

                    utils::grab_files(move |files| {
                        let mut message_builders = message_builder_state.clone();
                        message_builders.insert(id, attachments_message_builder.attachments(files));
                        attachment_set_message_builder(message_builders);
                    });
                },
                "Upload"
            }
            input {
                style: "flex-grow: 1",
                oninput: move |evt| {
                    let mut message_builders = message_builder_state.clone();
                    message_builders.insert(cx.props.channel_id.clone(), content_message_builder.clone().content(evt.value.clone()));
                    set_message_builders(message_builders);
                }
            },
            button {
                style: "width: 10%",
                onclick: move |_| {
                    let channel_id = cx.props.channel_id.clone();
                    let http = http.clone();
                    let message_builder = message_builder.clone();

                    cx.spawn(async move {
                        http.send_message(
                            &channel_id, message_builder.build(&http).await,
                        ).await;
                    })
                },
                "Send"
            }
        }
    })
}
