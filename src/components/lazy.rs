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


use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use js_sys::{Function, Array,};
use crate::prelude::*;


#[derive(Props)]
pub struct LazyProps<'a, 'b> {
    pub data: Vec<LazyNodes<'a, 'b>>,
    pub onnew: EventHandler<'a>
}

pub fn Lazy<'a, 'b>(cx: Scope<'a, LazyProps<'a, 'b>>) -> Element<'a> {
    let observer = cx.use_hook(|| {
        let options = IntersectionObserverInit::new();

        let func = Closure::<dyn Fn(Array)>::new(|entries: Array| {
            todo!()
        });

        let observer = IntersectionObserver::new_with_options(func.as_ref().unchecked_ref(), &options).unwrap();
        observer
    });

    todo!()
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    pub fn test(cx: Scope) -> Element {
        let data = use_state(&cx, Vec::<u8>::new);

        cx.render(rsx! {
            components::Lazy {
                data: data.get().iter().map(|&num| {
                    rsx! {
                        "{num}"
                    }
                }).collect(),
                onnew: move |_| {

                }
            }
        })
    }
}
