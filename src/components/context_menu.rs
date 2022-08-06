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
    let context_menu = use_context_menu(&cx);

    cx.render(rsx! {
        cx.props.buttons.iter().map(|(text, cell)| {
            let context_menu = context_menu.clone();

            rsx! {
                button {
                    onclick: move |_| {
                        let callback = cell.borrow_mut().take().unwrap();
                        cx.spawn(callback());
                        context_menu.set(None);
                    },
                    "{text}"
                }
            }
        })
    })
}

pub fn ContextMenu(cx: Scope) -> Element {
    let context_menu = use_context_menu(&cx);
    let modal = use_modal(&cx);

    cx.render(rsx! {
        context_menu.get().as_ref().map(|context_menu| {
            let buttons = match context_menu {
                ActiveContextMenu::Message { message_id, channel_id } => {
                    vec![
                        ("Delete Message", {
                            to_owned![modal, message_id, channel_id];

                            wrap_async(async move || {
                                move_variables![modal, message_id, channel_id];

                                modal.push_modal(ActiveModal::DeleteMessage { channel_id, message_id })
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
