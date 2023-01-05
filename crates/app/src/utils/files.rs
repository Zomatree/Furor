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


use js_sys::Uint8Array;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, HtmlInputElement, File, Blob};
use wasm_bindgen_futures::JsFuture;

pub fn grab_files<F: FnOnce(Vec<File>) + 'static>(callback: F) {
    let document = web_sys::window().unwrap().document().unwrap();
    let input = document.create_element("input").unwrap();
    input.set_attribute("accept", "").unwrap();
    input.set_attribute("type", "file").unwrap();
    input.set_attribute("style", "display: none").unwrap();

    let html_element = input.dyn_into::<HtmlElement>().unwrap();
    let callback_html_element = html_element.clone();
    let cb = Closure::once_into_js(move || {
        let files = callback_html_element.dyn_into::<HtmlInputElement>().unwrap().files();

        log::info!("{files:?}");

        if let Some(files) = files {
            let length = files.length();

            let files = (0..length)
                .map(|i| files.get(i).unwrap())
                .collect();

            callback(files);
        }
    });

    let listener = cb.as_ref().unchecked_ref();

    html_element.add_event_listener_with_callback("change", listener).unwrap();

    document.get_element_by_id("main").unwrap().append_child(&html_element).unwrap();

    html_element.click()
}


pub async fn read_file(file: &Blob) -> Vec<u8> {
    let buf = JsFuture::from(file.array_buffer()).await.unwrap();
    let array = Uint8Array::new(&buf);
    array.to_vec()
}
