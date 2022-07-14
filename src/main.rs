#![feature(async_closure)]
#![feature(let_chains)]
#![feature(never_type)]
#![feature(trait_alias)]
#![allow(non_snake_case)]
#![allow(clippy::derive_partial_eq_without_eq)]

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
    pub use dioxus::core::to_owned;
    pub use crate::*;
    pub use crate::types;
    pub use crate::utils::*;
    pub use crate::state::*;
    pub use crate::components;
    pub use crate::http::HTTPClient;
}

pub const API_URL: &str = "https://api.revolt.chat";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus::web::launch(pages::App);
}

#[cfg(test)]
mod tests {
    use comrak::nodes::{NodeValue, NodeLink};

    fn print_node<'a>(node: &'a comrak::nodes::AstNode<'a>, depth: usize) {
        let value = &node.data.borrow().value;
        let starter = "-".repeat(depth);

        print!("{starter} {value:?}");

        match &value {
            NodeValue::Text(bytes) => println!(" '{}'", std::str::from_utf8(bytes).unwrap()),
            NodeValue::Link(NodeLink { url, title }) => {
                if !url.is_empty() {
                    println!(" '{}'", std::str::from_utf8(url).unwrap())
                }

                if !title.is_empty() {
                    println!(" '{}'", std::str::from_utf8(title).unwrap())
                }
            }
            _ => println!()
        }

        for child in node.children() {
            print_node(child, depth + 1)
        }
    }

    #[test]
    fn markdown() {
        let text = "
# header 1

## header 2

__*foo*__

<https://google.com>

https://google.com
        ";
        let arena = comrak::Arena::new();

        let nodes = comrak::parse_document(&arena, text, &comrak::ComrakOptions::default());

        print_node(nodes, 0);
    }
}
