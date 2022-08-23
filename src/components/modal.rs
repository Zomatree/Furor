use crate::prelude::*;

#[derive(Props)]
struct InnerModalProps {
    pub title: &'static str,
    pub description: &'static str,
    pub buttons: Vec<(&'static str, utils::TakenAsyncFunc<(), ()>)>,
}

impl PartialEq for InnerModalProps {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn InnerModal(cx: Scope<InnerModalProps>) -> Element {
    let modal = use_modal(&cx);

    cx.render(rsx! {
        div {
            "{cx.props.title}",
            "{cx.props.description}",
            cx.props.buttons.iter().map(|(name, callback)| {
                let modal = modal.clone();

                rsx! {
                    button {
                        onclick: move |_| {
                            let callback = callback.borrow_mut().take().unwrap();
                            cx.spawn(callback());

                            modal.pop_modal();
                        },
                        "{name}",
                    }
                }
            })
        }
    })
}

pub fn Modal(cx: Scope) -> Element {
    let modals = use_read(&cx, MODALS);
    let http = use_read(&cx, HTTP).as_ref();

    cx.render(match modals.as_slice() {
        &[] => rsx! { None::<()> },
        modals => {
            rsx! {
                modals.iter().cloned().map(|modal| {
                    match modal {
                        ActiveModal::DeleteMessage { channel_id, message_id } => {
                            let http = http.cloned().unwrap();

                            rsx! {
                                InnerModal {
                                    title: "Delete Message?",
                                    description: "Are you sure you want to delete this message, this action cannot be undone.",
                                    buttons: vec![
                                        ("Ok", wrap_async(async move || {
                                                http.delete_message(&channel_id, &message_id).await;
                                        })),
                                        ("Cancel", wrap_async(async move || {}))
                                    ]
                                }
                            }
                        },
                        ActiveModal::React { channel_id, message_id } => {
                            let http = http.cloned().unwrap();
                            let modal = use_modal(&cx);
                            let emoji = use_state(&cx, String::new);

                            rsx! {
                                div {
                                    "React to Message",
                                    "Enter your emoji",
                                    input {
                                        oninput: |evt| {
                                            emoji.set(evt.value.clone())
                                        }
                                    },
                                    button {
                                        onclick: {
                                            let modal = modal.clone();

                                            move |_| {
                                                to_owned![channel_id, message_id, modal, http];

                                                let emoji = emoji.get().clone();

                                                cx.spawn(async move {
                                                    http.add_reaction(channel_id, message_id, emoji).await;
                                                });

                                                modal.pop_modal();
                                            }
                                        },
                                        "React",
                                    },
                                    button {
                                        onclick: {
                                            let modal = modal.clone();

                                            move |_| {
                                                modal.pop_modal();
                                            }
                                        },
                                        "Cancel"
                                    }
                                }
                            }
                        }
                    }
                })
            }
        }
    })
}
