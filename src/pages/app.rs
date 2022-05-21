use dioxus::prelude::*;
use crate::{pages::login::Login, prelude::*};

#[derive(Props, PartialEq)]
pub struct AppProps {
    pub api_url: String
}

pub fn App(cx: Scope<AppProps>) -> Element {
    let fut = use_future(&cx, (), |_| {
        let api_url = cx.props.api_url.clone();

        async move {
            let client = reqwest::Client::new();
            let res = client.get(api_url)
                .send()
                .await
                .unwrap()
                .json::<types::RevoltConfig>()
                .await
                .unwrap();
            res
        }
    });

    cx.render(match fut.value() {
        Some(revolt_config) => {
            rsx! {
                Login {
                    revolt_config: revolt_config.clone()
                }
            }
        },
        None => {
            rsx! {
                h1 {
                    "Loading..."
                }
            }
        }
    })
}
