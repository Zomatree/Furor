use js_sys::{eval, global, Reflect};
use wasm_bindgen::closure::Closure;
use std::sync::Mutex;

use crate::prelude::*;

#[derive(Props)]
pub struct HCaptchaProps<F: FnOnce(String) + 'static> {
    sitekey: String,
    complete_callback: Mutex<Option<F>>,
}

impl<F: FnOnce(String) + 'static> PartialEq for HCaptchaProps<F> {
    fn eq(&self, other: &Self) -> bool {
        self.sitekey == other.sitekey
    }
}

pub fn HCaptcha<F: FnOnce(String) + 'static>(cx: Scope<HCaptchaProps<F>>) -> Element {
    let sitekey = &cx.props.sitekey;

    cx.use_hook(|| {
        let oncomplete = &cx.props.complete_callback;
        let owned = oncomplete.lock().unwrap().take().unwrap();

        let closure = Closure::once_into_js(move || {
            let token = eval("hcaptcha.getResponse(globalThis.hcaptcha_id)").unwrap().as_string().unwrap();
            (owned)(token)
        });

        Reflect::set(&global(), &"hcaptcha_callback".into(), &closure).unwrap();
    });

    rsx!(cx, div {
        div {
            dangerous_inner_html: "<div id='h-captcha' data-sitekey='{sitekey}'></div>"
        },
        script {
            "globalThis.hcaptcha_id = hcaptcha.render('h-captcha', {{'callback': hcaptcha_callback}})"
        }
    })
}
