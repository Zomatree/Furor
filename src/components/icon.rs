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
pub struct IconProps {
    #[props(optional)]
    height: Option<u16>,

    #[props(optional)]
    width: Option<u16>,

    src: String,
}


pub fn Icon(cx: Scope<IconProps>) -> Element {
    let height = cx.props.height.unwrap_or(32);
    let width = cx.props.width.unwrap_or(32);

    cx.render(rsx! {
        img {
            src: "{cx.props.src}",
            height: "{height}",
            width: "{width}"
        }
    })
}
