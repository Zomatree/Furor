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
pub struct ButtonProps<'a> {
    onclick: EventHandler<'a, Event<MouseData>>,
    #[props(optional)]
    background: Option<bool>,
    #[props(optional)]
    style: Option<&'a str>,
    children: Element<'a>
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let background = cx.props.background.unwrap_or_default();
    let theme = use_theme(cx);

    let styling = if background {
        format!("background-color: {}", theme.secondary_header)
    } else {
        "background-color: transparent".to_string()
    };

    cx.render(rsx! {
        button {
            style: "border-radius: 6px; border-width: 0px; padding: 5px; color: #CFCFCF; {styling}; {cx.props.style.unwrap_or_default()}",
            onclick: |e| cx.props.onclick.call(e),
            &cx.props.children
        }
    })
}
