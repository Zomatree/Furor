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


use crate::prelude::*;
use comrak::{parse_document, Arena, nodes::{NodeValue, AstNode, NodeLink}};

// comrak gives us bytes as an output when the input is a string,
// this means its always going to be valid utf8 presuming comrak doesnt mess up
fn to_utf8(bytes: &[u8]) -> &str {
    unsafe {
        std::str::from_utf8_unchecked(bytes)
    }
}

#[derive(Props, PartialEq)]
pub struct MarkdownProps {
    text: String
}

fn render_markdown<'a, 'b>(node: &'b AstNode<'b>) -> LazyNodes<'a, 'b> {
    let lazy_nodes = node.children().map(|node| {
        let value = &node.data.borrow().value;

        log::debug!("{value:?}");

        match value {
            NodeValue::Text(text) => {
                let text = to_utf8(text).to_string();

                rsx! {
                    span { "{text}" }
                }
            },
            NodeValue::Heading(header) => {
                let inner = render_markdown(node);

                match header.level {
                    1 => rsx! {
                        h1 { inner }
                    },
                    2 => rsx! {
                        h2 { inner }
                    },
                    3 => rsx! {
                        h3 { inner }
                    },
                    4 => rsx! {
                        h4 { inner }
                    },
                    5 => rsx! {
                        h5 { inner }
                    },
                    _ => rsx! {
                        h6 { inner }
                    }
                }
            },
            NodeValue::Paragraph => {
                let inner = render_markdown(node);

                rsx! {
                    span { inner }
                }
            },
            NodeValue::Emph => {
                let inner = render_markdown(node);

                rsx! {
                    em { inner }
                }
            },
            NodeValue::Strikethrough => {
                let inner = render_markdown(node);

                rsx! {
                    s { inner }
                }
            },
            NodeValue::Strong => {
                let inner = render_markdown(node);

                rsx! {
                    strong { inner }
                }
            },
            NodeValue::Link(NodeLink { url, .. }) => {
                let url = to_utf8(url).to_string();

                rsx! {
                    a {
                        href: "{url}",
                        "{url}"
                    }
                }
            }
            _ => {
                let inner = render_markdown(node);

                rsx! {
                    "todo",
                    inner,
                    "todo"
                }
            }
        }
    });

    rsx! { lazy_nodes }
}

pub fn Markdown(cx: Scope<MarkdownProps>) -> Element {
    let arena = Arena::new();
    let root = parse_document(&arena, &cx.props.text, &comrak::ComrakOptions::default());

    cx.render(render_markdown(root))
}
