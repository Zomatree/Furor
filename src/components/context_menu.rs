use crate::prelude::*;

#[derive(Props)]
struct ContextMenuInnerProps {
    pub buttons: Vec<(&'static str, utils::TakenAsyncFunc<(), ()>)>
}

impl PartialEq for ContextMenuInnerProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn ContextMenuInner(cx: Scope<ContextMenuInnerProps>) -> Element {
    cx.render(rsx! {
        cx.props.buttons.iter().map(|(text, cell)| {
            rsx! {
                button {
                    onclick: move |_| {
                        let callback = cell.borrow_mut().take().unwrap();
                        cx.spawn(callback());
                    },
                    "{text}"
                }
            }
        })
    })
}

pub fn ContextMenu(cx: Scope) -> Element {
    let context_menu = use_context_menu(&cx);
    let http = use_http(&cx);

    cx.render(rsx! {
        context_menu.get().as_ref().map(|context_menu| {
            let buttons = match context_menu {
                ActiveContextMenu::Message { message_id, channel_id } => {
                    vec![
                        ("Delete Message", {
                            to_owned![message_id, channel_id, http];

                            wrap_async(async move || {
                                move_variables![message_id, channel_id, http];

                                http.delete_message(&channel_id, &message_id).await;
                            })
                        })
                    ]
                }
            };

            rsx! {
                ContextMenuInner {
                    buttons: buttons
                }
            }
        })
    })
}
