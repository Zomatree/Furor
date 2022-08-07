use crate::prelude::*;


#[derive(Props, PartialEq)]
pub struct MessageEditorProps {
    message_id: types::ULID,
    channel_id: types::ULID,
    initial_text: String
}

pub fn MessageEditor(cx: Scope<MessageEditorProps>) -> Element {
    let http = use_http(&cx);
    let set_currently_editing = use_set(&cx, CURRENTLY_EDITING);
    let content = use_state(&cx, || cx.props.initial_text.clone());

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
