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
            .categories
            .iter()
            .map(|category| {
                rsx! {
                    "{category.title}",
                    category.channels.iter()
                    .filter_map(|channel_id| channel_state.get(channel_id))
                    .map(|channel| {
                        match channel {
                            types::Channel::TextChannel(channel) => {
                                let cloned_id = channel.id.clone();

                                rsx! {
                                    button {
                                        key: "{cloned_id}",
                                        style: "display: flex; flex-direction: row",
                                        onclick: move |_| {
                                            set_channel(Some(channel.id.clone()));
                                            set_last_channel(cx.props.server_id.clone(), channel.id.clone());

                                            router.push_route(&format!("/server/{}/channel/{}", cx.props.server_id, channel.id), None, None);
                                        },
                                        span {
                                            "# ",
                                            "{channel.name}"
                                        },
                                    }
                                }
                            },
                            types::Channel::VoiceChannel(channel) => {
                                rsx! {
                                    button {
                                        key: "{channel.id}",
                                        style: "display: flex; flex-direction: row",
                                        span {
                                            "V ",
                                            "{channel.name}"
                                        },
                                    }
                                }
                            },
                            _ => unreachable!()
                        }
                    })
                }
            })
    })
}
