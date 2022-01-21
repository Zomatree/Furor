#![allow(non_snake_case)]
#![feature(concat_idents)]
#![feature(async_closure)]
#![feature(nll)]

pub mod pages;
pub mod components;
pub mod utils;
pub mod icons;
pub mod lib;
pub mod client;

pub use utils::*;

use dioxus::prelude::*;


fn app(cx: Scope) -> Element {
    println!("1");
    cx.push_future(async {
        client::connect("ENlW55s42xeiM-mGABZy2IrxctnLq70XASZvOrrMiLsUXpDXR0CBoVoUScM9Pmm8".to_string(), "https://api.revolt.chat".to_string()).await;

        let guard = client::CLIENT.get().unwrap().lock().unwrap();
        guard.start().await;
    });

    cx.render(rsx! {
        style { [include_str!("styles/global.css")] },
        pages::login()
    })
}

fn main() {
    dioxus::web::launch(app);
}
