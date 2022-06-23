use crate::prelude::*;

enum Channel<'a> {
    Dm(&'a types::DirectMessage),
    Group(&'a types::Group),
}

pub fn DirectMessageList(cx: Scope) -> Element {
    let channels_state = use_read(&cx, CHANNELS);
    let dm_channels_state = use_read(&cx, DM_CHANNELS);
    let user_state = use_read(&cx, USERS);
    let user_id = &use_read(&cx, USER).as_ref().unwrap().1;
    let server_members_state = use_read(&cx, SERVER_MEMBERS);

    rsx!(cx, div {
        style: "width: 232px; height: 100%",
        h1 {
            "Direct Messages"
        },
        button {
            "Home"
        },
        button {
            "Friends"
        },
        button {
            "Saved Notes"
        },
        h3 {
            "Conversations"
        },
        dm_channels_state.iter()
            .filter_map(|channel| {
                let channel = channels_state.get(channel)?;
                match channel {
                    types::Channel::DirectMessage(channel) => Some(Channel::Dm(channel)),
                    types::Channel::Group(channel) => Some(Channel::Group(channel)),
                    _ => None
                }
            })
            .map(|channel| {
                match channel {
                    Channel::Dm(dm) => {
                        let user_id = dm.get_recipient(user_id);
                        let user = user_state.get(user_id).unwrap();
                        let (username, avatar) = get_username_avatar(channels_state, server_members_state, user, &None, None);

                        rsx! {
                            div {
                                img {
                                    width: "32",
                                    height: "32",
                                    src: "{avatar}"
                                },
                                span {
                                    "{username}"
                                }
                            }
                        }
                    },
                    Channel::Group(group) => {
                        rsx! {
                            div {
                                group.icon.as_ref().map(|icon| {
                                    let url = icon.url();

                                    rsx! {
                                        img {
                                            width: "32",
                                            height: "32",
                                            src: "{url}"
                                        }
                                    }
                                }),
                                span {

                                }
                            }
                        }
                    }
                }
            })
    })
}
