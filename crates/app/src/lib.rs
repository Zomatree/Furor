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


#![feature(async_closure)]
#![feature(let_chains)]
#![feature(never_type)]
#![feature(trait_alias)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]
#![allow(non_snake_case)]
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod http;
pub mod pages;
pub mod components;
#[macro_use]
pub mod utils;
pub mod state;
pub mod websocket;

pub mod prelude {
    pub use dioxus::prelude::*;
    pub use dioxus_web::*;
    pub use fermi::prelude::*;
    pub use dioxus_router::*;
    pub use crate::*;
    pub use furor_core::types;
    pub use crate::utils::*;
    pub use crate::state::*;
    pub use crate::components;
    pub use crate::http::HTTPClient;
}

pub const API_URL: &str = "https://api.revolt.chat";

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(pages::App);
}

#[cfg(test)]
mod tests {
    use comrak::nodes::{NodeValue, NodeLink};

    fn print_node<'a>(node: &'a comrak::nodes::AstNode<'a>, depth: usize) {
        let value = &node.data.borrow().value;
        let starter = "-".repeat(depth);

        print!("{starter} {value:?}");

        match value {
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
