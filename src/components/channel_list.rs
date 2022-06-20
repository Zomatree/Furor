use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelListProps {
    server_id: types::ULID
}

pub fn ChannelList(cx: Scope<ChannelListProps>) -> Element {
    let server_state = use_read(&cx, SERVERS);
    let channel_state = use_read(&cx, CHANNELS);
    let set_channel = use_set(&cx, CURRENT_CHANNEL);
    let router = use_router(&cx);

    rsx!(cx, div {
        style: "display: flex; flex-direction: column; width: 232px",
        server_state[&cx.props.server_id]
            .channels
            .iter()
            .filter_map(|channel_id| channel_state.get(channel_id).cloned())
            .map(|channel| {
                match channel {
                    types::Channel::TextChannel { id, name, .. } => {
                        let cloned_id = id.clone();

                        rsx! {
                            button {
                                key: "{cloned_id}",
                                style: "display: flex; flex-direction: row",
                                onclick: move |_| {
                                    set_channel(Some(id.clone()));
                                    set_last_channel(cx.props.server_id.clone(), id.clone());

                                    router.push_route(&format!("/server/{}/channel/{}", cx.props.server_id, id), None, None);
                                },
                                span {
                                    "# ",
                                    "{name}"
                                },
                            }
                        }
                    },
                    types::Channel::VoiceChannel { id, name, .. } => {
                        rsx! {
                            button {
                                key: "{id}",
                                style: "display: flex; flex-direction: row",
                                span {
                                    "V ",
                                    "{name}"
                                },
                            }
                        }
                    },
                    _ => unreachable!()
                }
            })
    })
}
