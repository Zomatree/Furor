use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct ServerHeaderProps<'a> {
    server_id: &'a types::ULID
}

pub fn ServerHeader<'a>(cx: Scope<'a, ServerHeaderProps<'a>>) -> Element<'a> {
    let server_state = use_read(&cx, SERVERS);
    let revolt_config = use_config(&cx);

    let server = &server_state[cx.props.server_id];

    cx.render(rsx! {
        div {
            match &server.banner {
                Some(banner) => {
                    let url = banner.url(&revolt_config.features.autumn.url);

                    rsx! {
                        div {
                            style: "background-image: url(\"{url}\"); background-size: cover; background-position: center center; height: 120px; display: flex; flex-direction: column; justify-content: flex-end",
                            "{server.name}"
                        }
                    }
                },
                None => {
                    rsx! {
                        "{server.name}"
                    }
                }
            }
        }
    })
}