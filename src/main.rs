#![feature(explicit_generic_args_with_impl_trait)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(never_type)]
#![allow(non_snake_case)]

pub mod http;
pub mod pages;
pub mod components;
#[macro_use]
pub mod utils;
pub mod state;
pub mod types;
pub mod websocket;

pub mod prelude {
    pub use dioxus::prelude::*;
    pub use crate::*;
    pub use crate::types;
    pub use crate::utils::*;
    pub use crate::state::*;
    pub use crate::components;
    pub use crate::http::HTTPClient;
}

pub const API_URL: &str = "https://api.revolt.chat";
pub const AUTUMN_URL: &str = "autumn.revolt.chat";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(pages::App);
}
