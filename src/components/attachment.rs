use dioxus::prelude::*;
use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct AttachmentProps {
    asset: types::Asset
}

pub fn Attachment(cx: Scope<AttachmentProps>) -> Element {
    let AttachmentProps { asset: types::Asset { size, filename, metadata, .. } } = cx.props;
    let url = cx.props.asset.url();

    rsx!(cx, div {
        div {
            style: "padding: 12px; display: flex; flex-direction: row",
            div {
                style :"display: flex; flex-direction: column",
                span {
                    "{filename}"
                },
                span {
                    "{size}"
                }
            },
            a {
                href: "{url}",
                "Download"
            }
        },
        match metadata {
            types::AssetMetadata::File | types::AssetMetadata::Text => None,
            types::AssetMetadata::Audio => {
                Some(rsx! {
                    audio {
                        src: "{url}"
                    }
                })
            },
            &types::AssetMetadata::Image { width, height } => {
                Some(rsx! {
                    div {
                        style: "--width: {width}; --height: {height}; aspect-ratio: {width}/{height}; max-width: min(var(--width), 400); max-height: min(var(--height), 300); display: grid; overflow; hidden",
                        width: "{width}",
                        height: "{height}",
                        img {
                            style: "max-width: 100%; max-height: 100%; overflow: hidden; grid-area: 1 / 1 / auto / auto; overflow: hidden; object-fit: contain; object-position: left center",
                            src: "{url}",
                            width: "{width}",
                            height: "{height}",
                            alt: "{filename}"
                        }
                    }
                })
            },
            &types::AssetMetadata::Video { width, height } => {
                Some(rsx! {
                    div {
                        style: "--width: {width}; --height: {height}; aspect-ratio: {width}/{height}; max-width: min(var(--width), 400); max-height: min(var(--height), 300); display: grid; overflow; hidden",
                        width: "{width}",
                        height: "{height}",
                        video {
                            style: "max-width: 100%; max-height: 100%; overflow: hidden; grid-area: 1 / 1 / auto / auto; overflow: hidden; object-fit: contain; object-position: left center",
                            src: "{url}",
                            width: "{width}",
                            height: "{height}",
                            alt: "{filename}",
                            controls: "true"
                        }
                    }
                })
            },
        }
    })
}
