/* Copyright (C) 2022-current  Zomatree <me@zomatree.live>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/. */


use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct AttachmentProps {
    asset: types::Asset
}

pub fn Attachment(cx: Scope<AttachmentProps>) -> Element {
    let revolt_config = use_config(&cx);

    let AttachmentProps { asset: types::Asset { size, filename, metadata, .. } } = cx.props;
    let url = cx.props.asset.url(&revolt_config.features.autumn.url);

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
            types::AssetMetadata::Image { width, height } => {
                Some(rsx! {
                    div {
                        style: "--width: {width}; --height: {height}; max-width: min(var(--width), 400); max-height: min(var(--height), 300); display: grid; overflow; hidden",
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
            types::AssetMetadata::Video { width, height } => {
                Some(rsx! {
                    div {
                        style: "--width: {width}; --height: {height}; max-width: min(var(--width), 400); max-height: min(var(--height), 300); display: grid; overflow; hidden",
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
