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
pub struct MessageEditorProps<'a> {
    message_id: &'a types::ULID,
    channel_id: &'a types::ULID,
    initial_text: String
}

pub fn MessageEditor<'a>(cx: Scope<'a, MessageEditorProps<'a>>) -> Element<'a> {
    let http = use_http(cx);
    let set_currently_editing = use_set(cx, CURRENTLY_EDITING);
    let content = use_state(cx, || cx.props.initial_text.clone());

    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; width: 100%",
            input {
                style: "width: 100%; font-size: 16px",
                value: format_args!("{}", content.get()),
                oninput: |evt| content.set(evt.value.clone())
            },
            div {
                style: "display: flex; flex-direction: row; justify-content: flex-end",
                button {
                    onclick: move |_| {
                        let http = http.clone();
                        let content = content.get().clone();
                        let channel_id = cx.props.channel_id.clone();
                        let message_id = cx.props.message_id.clone();
                        let set_currently_editing = set_currently_editing.clone();

                        cx.spawn(async move {
                            let message = utils::MessageBuilder::new()
                                .content(content)
                                .build(&http)
                                .await;

                            http.edit_message(&channel_id, &message_id, message).await;
                            set_currently_editing(None)
                        });
                    },
                    "Save"
                },
                button {
                    onclick: move |_| {
                        set_currently_editing(None)
                    },
                    "Cancel"
                }
            }
        }
    })
}
