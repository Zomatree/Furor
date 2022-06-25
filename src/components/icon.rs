use crate::prelude::*;

#[derive(Props, PartialEq)]
pub struct IconProps {
    #[props(optional)]
    height: Option<u16>,

    #[props(optional)]
    width: Option<u16>,

    src: String,
}


pub fn Icon(cx: Scope<IconProps>) -> Element {
    let height = cx.props.height.unwrap_or(32);
    let width = cx.props.width.unwrap_or(32);

    cx.render(rsx! {
        img {
            src: "{cx.props.src}",
            height: "{height}",
            width: "{width}"
        }
    })
}
