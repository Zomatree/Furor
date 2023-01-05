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


use std::{rc::Rc, cell::Cell, process::Output, pin::Pin};

use futures::Future;
use wasm_bindgen::{JsValue, prelude::Closure, JsCast};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit, window};
use js_sys::{Function, Array};
use rand::{thread_rng, distributions::Alphanumeric, Rng};
use wasm_bindgen_futures::{spawn_local};

use crate::prelude::*;


#[derive(Props)]
pub struct VirtualListProps<'a> {
    pub data: Vec<Element<'a>>,
    pub style: &'a str,
    pub onnew: Cell<Option<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static>>>
}

pub fn VirtualList<'a>(cx: Scope<'a, VirtualListProps<'a>>) -> Element<'a> {
    let element_id_state = use_state(cx, || None::<(bool, String)>);
    let document = cx.use_hook(|| window().unwrap().document().unwrap());
    let event = cx.use_hook(|| Rc::new(cx.props.onnew.take().unwrap()));

    let element_id = match element_id_state.get() {
        Some((true, id)) => {
            let options = IntersectionObserverInit::new();
            let event = event.clone();

            let mut func = Closure::<dyn Fn(Array)>::new(move |entries: Array| {
                log::info!("{entries:?}");
                if entries.get(0).unchecked_into::<IntersectionObserverEntry>().intersection_ratio() <= 0.0 {
                    spawn_local(event());
                }
            });

            let observer = IntersectionObserver::new_with_options(func.as_ref().unchecked_ref(), &options).unwrap();

            func.forget();

            observer.observe(&document.get_element_by_id(id).unwrap());
            element_id_state.set(Some((false, id.clone())));
            id.clone()
        },
        None => {
            let mut rng = thread_rng();
            let element_id = (0..7).map(|_| rng.sample(Alphanumeric) as char).collect::<String>();
            element_id_state.set(Some((true, element_id.clone())));
            element_id
        },
        Some((_, id)) => id.clone()
    };

    cx.render(rsx! {
        div {
            id: "{element_id}",
            style: cx.props.style,
            cx.props.data.clone().into_iter()
        }
    })
}

pub mod test {
    use futures::Future;

    use crate::prelude::*;
    use super::VirtualListProps;

    use std::{cell::Cell, pin::Pin};

    pub fn test(cx: Scope) -> Element {
        let data = use_state(cx, Vec::<u8>::new);

        let cb = Cell::new(Some(Box::new({
            let data = data.clone();

            move || {
                let data = data.clone();

                Box::pin(async move {
                    data.with_mut(|v| {
                        let last = *v.last().unwrap();

                        v.extend(last..last + 10)
                    })
                }) as Pin<Box<dyn Future<Output = ()>>>
            }
        }) as Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>> + 'static>));

        cx.render(rsx! {
            cx.component(components::VirtualList, VirtualListProps::builder().data(data.get().iter().map(|&num| {
                cx.render(rsx! {
                    "{num}"
                })
            }).collect()).onnew(cb).style("display: flex; flex-direction: column").build(), "VirtualList")
        })
    }
}
