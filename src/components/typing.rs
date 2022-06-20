use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct TypingProps {
    channel_id: types::ULID
}

pub fn Typing(cx: Scope<TypingProps>) -> Element {
    let typing_state = use_read(&cx, TYPING);
    let user_state = use_read(&cx, USERS);

    rsx!(cx, div {
        typing_state.get(&cx.props.channel_id).map(|currently_typing| {
            if currently_typing.is_empty() {
                return rsx! { None::<()> }
            };

            let mut avatars = vec![];
            let mut names = vec![];

            for user_id in currently_typing {
                let user = &user_state[user_id];
                let (username, avatar) = get_username_avatar(&cx, user, &None, &cx.props.channel_id);

                names.push(username);
                avatars.push(avatar);
            };

            let formatted_string = match names.len() {
                1 => {
                    format!("{} is", names[0])
                },
                2 => {
                    format!("{} and {} are", names[0], names[1])
                },
                _ => {
                    let last = names.pop().unwrap();

                    format!("{} and {} are", names.join(", "), last)
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
