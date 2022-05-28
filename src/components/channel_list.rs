use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelListProps<'a> {
    server_id: &'a types::ULID
}

pub fn ChannelList<'a>(cx: Scope<'a, ChannelListProps<'a>>) -> Element<'a> {
    let server_state = use_read(&cx, SERVERS);
    let channel_state = use_read(&cx, CHANNELS);

    rsx!(cx, div {
        style: "display: flex; flex-direction: column",
        server_state[cx.props.server_id]
            .channels
            .iter()
            .filter_map(|channel_id| channel_state.get(channel_id).cloned())
            .map(|channel| {
                match channel {
                    types::Channel::TextChannel { id, name, .. } => {
                        rsx! {
                            div {
                                key: "{id}",
                                style: "display: flex; flex-direction: row",
                                span {
                                    "# ",
                                    "{name}"
                                },
                            }
                        }
                    },
                    types::Channel::VoiceChannel { id, name, .. } => {
                        rsx! {
                            div {
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
