use crate::prelude::*;

#[derive(Default)]
pub struct MessageBuilder {
    content: Option<String>,
    attachments: Option<Vec<types::ULID>>,
    embeds: Option<Vec<types::SendableEmbed>>,
    replies: Option<Vec<types::Reply>>,
    masquerade: Option<types::Masquerade>
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn attachments(mut self, attachments: Vec<types::ULID>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn embeds(mut self, embeds: Vec<types::SendableEmbed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub fn replies(mut self, replies: Vec<types::Reply>) -> Self {
        self.replies = Some(replies);
        self
    }

    pub fn masquerade(mut self, masquerade: types::Masquerade) -> Self {
        self.masquerade = Some(masquerade);
        self
    }

    pub fn build(self) -> types::SendMessage {
        let Self { content, attachments, embeds, replies, masquerade } = self;

        types::SendMessage {
            content,
            attachments: attachments.unwrap_or_default(),
            embeds: embeds.unwrap_or_default(),
            replies: replies.unwrap_or_default(),
            masquerade
        }
    }
}
