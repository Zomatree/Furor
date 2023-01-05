use chumsky::{prelude::*, primitive::Container};


#[derive(Clone, Debug)]
pub enum Node {
    Text(String),
    Heading(usize, Box<Self>),
    Bold(Box<Node>),
    Italics(Box<Node>),
    InlineCodeblock(String),
    Codeblock(Option<String>, String),
    UserMention(String),
    ChannelMention(String),
    Emoij(String),
    Span(Vec<Node>)
}

impl Node {
    fn expect_text(self) -> String {
        match self {
            Node::Text(text) => text,
            _ => panic!("expected Node::Text")
        }
    }
}

pub fn parser() -> impl Parser<char, Node, Error = Simple<char>> {
    recursive(|tree| {
        let text = filter(|&c| c != '\n').repeated().at_least(1).collect::<String>()
            .map(Node::Text)
            .labelled("text");

        let heading = just('#')
            .repeated()
            .at_least(1)
            .at_most(6)
            .then_ignore(just(' ').repeated())
            .then(tree.clone().map(Box::new))
            .map(|(header, node)| Node::Heading(header.len(), node))
            .labelled("heading");

        let inline_codeblock = just('`')
            .then(text)
            .then_ignore(just('`'))
            .map(|(_, node)| Node::InlineCodeblock(node.expect_text()))
            .labelled("inline_codeblock");

        let codeblock = just("```")
            .then(text::ident::<char, Simple<char>>().or_not())
            .then_ignore(just('\n'))
            .then(text
                .repeated()
                .collect::<Vec<Node>>())
            .then_ignore(just('\n').or_not())
            .then_ignore(just("```"))
            .map(|((_, lang), nodes)| Node::Codeblock(lang, nodes.into_iter().map(Node::expect_text).collect::<Vec<String>>().join("\n")))
            .labelled("codeblock");

        let bold = just("**")
            .then(tree.clone())
            .then_ignore(just("**"))
            .map(|(_, node)| node)
            .map(Box::new)
            .map(Node::Bold)
            .labelled("bold");

        let italics = tree.clone()
            .delimited_by(just('*'), just('*'))
            .map(Box::new)
            .map(Node::Italics)
            .labelled("italics");

        let user_mention = just("<@")
            .then(none_of(">\n").repeated().collect::<String>())
            .then_ignore(just('>'))
            .map(|(_, id)| Node::UserMention(id))
            .labelled("user_mention");

        let channel_mention = just("<#")
            .then(none_of(">\n").repeated().collect::<String>())
            .then_ignore(just('>'))
            .map(|(_, id)| Node::ChannelMention(id))
            .labelled("channel_mention");

        let emoji = just(':')
            .then(none_of(":\n").repeated().collect::<String>())
            .then_ignore(just(':'))
            .map(|(_, name)| Node::Emoij(name))
            .labelled("emoji");

        let scheme = none_of(':').repeated().collect::<String>().then_ignore(just(':'));
        let user_info = none_of('@').repeated().collect::<String>().then_ignore(just('@'));

        let host = user_info
            .or_not()
            .then(none_of('/').repeated().collect::<String>().then_ignore(just('/')))
            .then(just(':').then(text::int(10)).map(|(_, i)| i).or_not());

        let query = just('?').then(none_of('#').repeated().collect::<String>().then_ignore(just('#'))).map(|(_, i)| i);
        let fragment = just('#').then(none_of("\n ").repeated().collect::<String>()).map(|(_, i)| i);

        let path = none_of("\n ").repeated().collect::<String>();

        let uri = scheme
            .then(just("//").then(host).map(|(_, i)| i).or_not())
            .then(path)
            .then(query.or_not())
            .then(fragment.or_not())
            .map(|((((a, b), c), d), e)| Node::Span(vec![]));

        heading
            .or(bold)
            .or(italics)
            .or(inline_codeblock)
            .or(codeblock)
            .or(user_mention)
            .or(channel_mention)
            .or(emoji)
            .or(uri)
            .or(text)
            .padded()
    })
}

pub fn parse(input: &str) -> Result<Node, Vec<Simple<char>>> {
    parser()
        .repeated()
        .map(Node::Span)
        .parse(input)
}
