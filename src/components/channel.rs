use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ChannelProps<'a> {
    pub channel_id: &'a types::ULID,
}

pub fn Channel<'a>(cx: Scope<'a, ChannelProps<'a>>) -> Element<'a> {
    let channel_state = use_read(&cx, CHANNELS);
    let user_state = use_read(&cx, USERS);

    let (_, user_id) = use_read(&cx, USER).as_ref().unwrap();

    let name = match &channel_state[&cx.props.channel_id] {
        types::Channel::DirectMessage(dm) => {
            let recipient_id = dm.get_recipient(user_id);
            &user_state[recipient_id].username
        },
        channel => channel.name().unwrap()
    };

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%",
            div {
                style: "height: 48px; width: 100%",
                "{name}"
            }
            components::ChannelMessages {
                channel_id: cx.props.channel_id,
            },
            components::Typing {
                channel_id: cx.props.channel_id,

            }
            components::MessageArea {
                channel_id: cx.props.channel_id,
            }
        }
    })
}
