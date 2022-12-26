use crate::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    onclick: EventHandler<'a, Event<MouseData>>,
    children: Element<'a>
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        button {
            style: "border-radius: 5px; border-width: 0px; border-color: #343434; padding: 5px; background-color: #232323; color: #CFCFCF",
            onclick: |e| cx.props.onclick.call(e),
            &cx.props.children
        }
    })
}
