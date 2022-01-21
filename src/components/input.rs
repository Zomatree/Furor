use dioxus::prelude::*;
use crate::styled;

styled!(StateInputInner, input, "
    background: #0b0e14;
    z-index: 1;
    font-size: 1rem;
    padding: 8px 16px;
    border-radius: var(--border-radius);
    font-family: inherit;
    color: #b3b1ad;
    border: none;
    outline: transparent solid 2px;
");

#[derive(Props)]
pub struct InputProps<'a> {
    pub state: UseState<'a, String>,
    pub children: Element<'a>,

    pub name: &'a str,
    pub r#type: &'a str,
    pub placeholder: &'a str,
}

pub fn StateInput<'a>(cx: Scope<'a, InputProps<'a>>) -> Element {
    cx.render(rsx! (
        StateInputInner {
            //name: "{cx.props.name}",
            //r#type: "{cx.props.r#type}",
            //placeholder: "{cx.props.placeholder}",

            //oninput: move |evt| {
            //    cx.props.state.set(evt.value.clone())
            //},

            &cx.props.children,
        }
    ))
}
