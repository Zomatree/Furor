use crate::prelude::*;

#[derive(Props)]
pub struct InputProps<'a> {
    oninput: EventHandler<'a, Event<FormData>>,
    placeholder: &'a str,
    children: Element<'a>
}

pub fn Input<'a>(cx: Scope<'a, InputProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        input {
            style: "border-radius: 5px; border-width: 1px; border-color: #343434; padding: 5px; background-color: #232323; color: #CFCFCF",
            placeholder: cx.props.placeholder,
            oninput: |e| cx.props.oninput.call(e),
            &cx.props.children
        }
    })
}
