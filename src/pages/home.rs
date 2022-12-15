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

pub fn Home(cx: Scope) -> Element {
    redirect_to_login(cx);

    cx.render(match get_local_storage_user().is_some() {
        true => rsx!(div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: row",
            components::ServerList {},
            div {
                style: "display: flex; flex-direction: row; flex-grow: 1",
                components::DirectMessageList {},
                h1 {
                    "Welcome to Revolt"
                }
            }
        }),
        false => rsx!(components::Loading {})
    })
}
