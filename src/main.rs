#![feature(explicit_generic_args_with_impl_trait)]
#![feature(async_closure)]
#![feature(let_chains)]
#![allow(non_snake_case)]

pub mod http;
pub mod components;
pub mod pages;
#[macro_use]
pub mod utils;
pub mod state;
pub mod types;
pub mod websocket;

pub mod prelude {
    pub use crate::utils::*;
    pub use crate::types;
    pub use crate::state::*;
    pub use crate::http::HTTPClient;
    pub use crate::*;
    pub use crate::components;
}

pub const API_URL: &str = "https://api.revolt.chat";
pub const AUTUMN_URL: &str = "autumn.revolt.chat";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch_with_props(pages::App, pages::app::AppProps {
        api_url: String::from(API_URL)
    }, |v| v);
}
