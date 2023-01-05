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

#[derive(Props)]
pub struct TextProps<'a> {
    #[props(default)]
    style: Option<&'a str>,
    #[props(default)]
    version: Option<&'a str>,
    children: Element<'a>
}

pub fn Text<'a>(cx: Scope<'a, TextProps<'a>>) -> Element<'a> {
    let theme = use_theme(cx);

    let colour = match cx.props.version.unwrap_or("primary") {
        "secondary" => &theme.secondary_foreground,
        "tertiary" => &theme.tertiary_foreground,
        _ => &theme.foreground,
    };

    cx.render(rsx! {
        div {
            style: "color: {colour}; {cx.props.style.unwrap_or_default()}",
            &cx.props.children
        }
    })
}
