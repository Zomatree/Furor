use crate::prelude::*;

pub fn Loading(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            span {
                "Loading..."
            }
        }
    })
}
