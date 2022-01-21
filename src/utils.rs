#[macro_export]
macro_rules! println {
    ($string:expr, $( $arg:expr ),+) => {{
        use web_sys::console::log_1;
        log_1(&format!("{}\n", format!($string, $( $arg ),*)).into())
    }};
    ($string:expr) => {{
        use web_sys::console::log_1;
        log_1(&format!("{}\n", $string).into())
    }}
}


#[macro_export]
macro_rules! styled {
    ($name:ident, $element:ident, $style:expr) => {
        #[inline_props]
        pub fn $name<'a>(cx: Scope, children: Element<'a>) -> Element<'a> {
            cx.render(rsx! {
                $element {
                    style: $style,
                    children
                }
            })
        }
    }
}
