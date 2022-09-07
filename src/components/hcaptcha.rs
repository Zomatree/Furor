/* Copyright (C) 2022-current  Zomatree <me@zomatree.live>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/. */


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
