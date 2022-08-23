use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct MessageProps {
    pub message_id: types::ULID,
    pub channel_id: types::ULID
}

pub fn Message(cx: Scope<MessageProps>) -> Element {
    let message_state = use_read(&cx, MESSAGES);
    let channel_state = use_read(&cx, CHANNELS);
    let server_members = use_read(&cx, SERVER_MEMBERS);
    let revolt_config = use_config(&cx);
    let message_builder_state = use_read(&cx, MESSAGE_BUILDERS);
    let set_message_builders = use_set(&cx, MESSAGE_BUILDERS);
    let user_state = use_read(&cx, USERS);
    let currently_editing = use_read(&cx, CURRENTLY_EDITING).as_ref();
    let set_currently_editing = use_set(&cx, CURRENTLY_EDITING);
    let context_menu = use_context_menu(&cx);

    let modal = utils::use_modal(&cx);

    let message = message_state
        .get(&cx.props.channel_id)?
        .get(&cx.props.message_id)?;

    let types::Message { content, author, attachments, channel, masquerade, replies, edited, id, .. } = message;

    let user = user_state.get(author).unwrap();
    let (username, avatar) = get_username_avatar(channel_state, server_members, revolt_config, user, masquerade, Some(channel));
    let content = content.clone().unwrap_or_default();
    let created_at = cx.use_hook(|| format_datetime(&id.timestamp()));  // only needs to be calculated once

    let message_builder = match message_builder_state.get(&cx.props.channel_id) {
        Some(message_builder) => message_builder.clone(),
        None => {
            let message_builder = utils::MessageBuilder::new();
            let mut message_builders = message_builder_state.clone();
            message_builders.insert(cx.props.channel_id.clone(), message_builder);
            message_builders.get(&cx.props.channel_id).unwrap().clone()
        }
    };

    let set_message_builder = move |builder| {
        let mut message_builders = message_builder_state.clone();
        message_builders.insert(cx.props.channel_id.clone(), builder);
        set_message_builders(message_builders);
    };

    cx.render(rsx! {
        div {
            oncontextmenu: move |_| {
                context_menu.set(Some(ActiveContextMenu::Message { message_id: id.clone(), channel_id: channel.clone() }));
            },
            prevent_default: "oncontextmenu",
            style: "display: flex; padding: 0.125rem; margin-top: 12px; padding-inline-end: 16px; flex-direction: column",
            div {
                style: "display: flex; flex-direction: column",
                replies
                    .iter()
                    .map(|reply| {
                        rsx! {
                            div {
                                style: "gap: 8px; min-width: 8px; display: flex; margin-inline: 30px 12px; font-size: 0.8em",
                                key: "{reply}",
                                components::Reply {
                                    message_id: reply.clone(),
                                    channel_id: cx.props.channel_id.clone(),
                                    message_mentions: replies.clone()
                                }
                            }
                        }
                    })
            },
            div {
                style: "display: flex; flex-direction: row",
                div {
                    style: "display: flex; flex-direction: row; width: 62px",
                    img {
                        src: "{avatar}",
                        width: "44",
                        height: "44"
                    },
                },
                div {
                    style: "display: flex; flex-direction: column; justify-content: center; flex-grow: 1",
                    span {
                        style: "gap: 8px; display: flex; align-items: center",
                        span { "{username}" },
                        user.bot.is_some().then(|| rsx! {
                            span {
                                "[BOT]"
                            }
                        }),
                        time { "{created_at}" },
                        edited.is_some().then(|| rsx! {
                            span {
                                style: "font-size: 10px",
                                "(edited)"
                            }
                        })
                    },
                    if Some(id) == currently_editing {
                        rsx! {
                            components::MessageEditor {
                                message_id: id.clone(),
                                channel_id: channel.clone(),
                                initial_text: content
                            }
                        }
                    } else {
                        rsx! {
                            components::Markdown {
                                text: content
                            }
                        }
                    }
                },
            }
            attachments.iter().cloned().enumerate().map(|(i, asset)| {
                rsx! {
                    div {
                        key: "{i}",
                        components::Attachment {
                            asset: asset
                        }
                    }
                }
            }),
            components::MessageReactions {
                channel_id: cx.props.channel_id.clone(),
                message_id: cx.props.message_id.clone()
            },
            div {
                style: "display: flex; flex-direction: row; justify-content: flex-end",
                button {
                    onclick: {
                        let modal = modal.clone();

                        move |_| {
                            modal.push_modal(utils::ActiveModal::React {
                                channel_id: cx.props.channel_id.clone(),
                                message_id: cx.props.message_id.clone()
                            });
                        }
                    },
                    "react"
                },
                button {
                    onclick: {
                        let modal = modal.clone();

                        move |_| {
                            modal.push_modal(utils::ActiveModal::DeleteMessage {
                                channel_id: cx.props.channel_id.clone(),
                                message_id: cx.props.message_id.clone()
                            });
                        }
                    },
                    "delete"
                },
                button {
                    onclick: move |_| {
                        set_message_builder(message_builder.clone().push_reply(types::Reply { id: id.clone(), mention: false }))
                    },
                    "reply"
                },
                button {
                    onclick: move |_| {
                        set_currently_editing(Some(id.clone()))
                    },
                    "edit"
                }
            }
        }
    })
}
