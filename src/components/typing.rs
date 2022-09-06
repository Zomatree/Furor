use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct TypingProps<'a> {
    channel_id: &'a types::ULID
}

pub fn Typing<'a>(cx: Scope<'a, TypingProps<'a>>) -> Element<'a> {
    let typing_state = use_read(&cx, TYPING);
    let user_state = use_read(&cx, USERS);
    let server_member_state = use_read(&cx, SERVER_MEMBERS);
    let channel_state = use_read(&cx, CHANNELS);
    let revolt_config = use_config(&cx);

    rsx!(cx, div {
        typing_state.get(cx.props.channel_id).map(|currently_typing| {
            if currently_typing.is_empty() {
                return rsx! { None::<()> }
            };

            let mut avatars = vec![];
            let mut names = vec![];

            for user_id in currently_typing {
                let user = &user_state[user_id];
                let (username, avatar) = get_username_avatar(channel_state, server_member_state, revolt_config, user, &None, Some(cx.props.channel_id));

                names.push(username);
                avatars.push(avatar);
            };

            let formatted_string = match names.as_slice() {
                [name] => {
                    format!("{} is", name)
                },
                names => {
                    let last = names.last().unwrap();

                    format!("{} and {} are", names[..names.len() - 1].join(", "), last)
                }
            };

            rsx! {
                div {
                    style: "z-index: 2",
                    avatars.iter().map(|url| {
                        rsx! {
                            img {
                                key: "{url}",
                                src: "{url}",
                                width: "16",
                                height: "16"
                            }
                        }
                    }),
                    "{formatted_string} typing..."
                }
            }
        })
    })
}
