use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct TypingProps<'a> {
    channel_id: &'a types::ULID,
    server_id: &'a types::ULID
}

pub fn Typing<'a>(cx: Scope<'a, TypingProps<'a>>) -> Element<'a> {
    let typing_state = use_read(&cx, TYPING);
    let user_state = use_read(&cx, USERS);

    rsx!(cx, div {
        typing_state.get(cx.props.channel_id).map(|currently_typing| {
            let mut avatars = vec![];
            let mut names = vec![];

            for user_id in currently_typing {
                let user = &user_state[user_id];
                let (username, avatar) = get_username_avatar(&cx, user, &None, cx.props.channel_id);

                names.push(username);
                avatars.push(avatar);
            };

            let formatted_string = names.join(", ");

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
                    "{formatted_string} are typing..."
                }
            }
        })
    })
}
