use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelProps {
    pub channel_id: types::ULID,

    #[props(optional)]
    pub name: Option<String>
}

pub fn Channel(cx: Scope<ChannelProps>) -> Element {
    let channels = use_read(&cx, CHANNELS);

    let name = cx.props.name
        .clone()
        .or_else(|| channels[&cx.props.channel_id].name())
        .unwrap();

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%",
            div {
                style: "height: 48px; width: 100%",
                "{name}"
            }
            components::ChannelMessages {
                channel_id: cx.props.channel_id.clone(),
            },
            components::Typing {
                channel_id: cx.props.channel_id.clone(),

            }
            components::MessageArea {
                channel_id: cx.props.channel_id.clone(),
            }
        }
    })
}
