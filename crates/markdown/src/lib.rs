#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
pub struct MarkdownParser;

mod parser;

#[test]
fn test() {
    let output = MarkdownParser::parse(Rule::file, "# heading 1");

    println!("{output:?}");
}

#[derive(Debug)]
struct Reply;
#[derive(Debug)]
struct Masquerade;
#[derive(Debug)]
struct Attachment;

#[derive(Debug, Default)]
struct Message {
    content: Option<String>,
    replies: Option<Vec<Reply>>,
    masquerade: Option<Masquerade>,
    attachments: Option<Vec<Attachment>>
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn replies(mut self, replies: Vec<Reply>) -> Self {
        self.replies = Some(replies);
        self
    }
}
