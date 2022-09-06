use crate::prelude::*;


#[derive(Props, PartialEq)]
pub struct MessageReactionsProps<'a> {
    message_id: &'a types::ULID,
    channel_id: &'a types::ULID
}

#[derive(Props, PartialEq)]
pub struct MessageReactionProps<'a> {
    channel_id: &'a types::ULID,
    message_id: &'a types::ULID,
    emoji: String,
    count: usize,
    reacted: bool,
}

pub fn MessageReaction<'a>(cx: Scope<'a, MessageReactionProps<'a>>) -> Element<'a> {
    let http = use_http(&cx);

    cx.render(rsx! {
        button {
            onclick: move |_| {
                let http = http.clone();
                let channel_id = cx.props.channel_id.clone();
                let message_id = cx.props.message_id.clone();
                let emoji = cx.props.emoji.clone();
                let reacted = cx.props.reacted;

                cx.spawn(async move {
                    if reacted {
                        http.remove_reaction(&channel_id, &message_id, &emoji).await;
                    } else {
                        http.add_reaction(&channel_id, &message_id, &emoji).await;
                    }
                });
            },
            border_color: format_args!("{}", if cx.props.reacted { "yellow" } else { "buttonborder" }),
            "{cx.props.emoji} {cx.props.count}",
        }
    })
}

pub fn MessageReactions<'a>(cx: Scope<'a, MessageReactionsProps<'a>>) -> Element<'a> {
    let message_state = use_read(&cx, MESSAGES);
    let user_id = use_user(&cx).1;

    let message = &message_state[cx.props.channel_id][cx.props.message_id];
    let message_reactions = &message.reactions;

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: row",
            message_reactions.iter().map(|(emoji, users)| rsx! {
                MessageReaction {
                    emoji: emoji.clone(),
                    channel_id: cx.props.channel_id,
                    message_id: cx.props.message_id,
                    count: users.len(),
                    reacted: users.contains(user_id)
                }
            })
        }
    })
}