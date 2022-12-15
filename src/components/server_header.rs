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
pub struct ServerHeaderProps<'a> {
    server_id: &'a types::ULID
}

pub fn ServerHeader<'a>(cx: Scope<'a, ServerHeaderProps<'a>>) -> Element<'a> {
    let server_state = use_read(cx, SERVERS);
    let revolt_config = use_config(cx);

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
