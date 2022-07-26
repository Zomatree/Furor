use crate::prelude::*;
use web_sys::File;

#[derive(Default, Clone)]
pub struct MessageBuilder {
    pub content: Option<String>,
    pub attachments: Option<Vec<File>>,
    pub embeds: Option<Vec<types::SendableEmbed>>,
    pub replies: Option<Vec<types::Reply>>,
    pub masquerade: Option<types::Masquerade>
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn push_content(self, content: String) -> Self {
        let mut old_content = self.content.clone().unwrap_or_default();
        old_content.push_str(&content);
        self.content(old_content)
    }

    pub fn attachments(mut self, attachments: Vec<File>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn push_attachment(self, attachment: File) -> Self {
        let mut attachments = self.attachments.clone().unwrap_or_default();
        attachments.push(attachment);
        self.attachments(attachments)
    }

    pub fn embeds(mut self, embeds: Vec<types::SendableEmbed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub fn push_embed(self, embed: types::SendableEmbed) -> Self {
        let mut embeds = self.embeds.clone().unwrap_or_default();
        embeds.push(embed);
        self.embeds(embeds)
    }

    pub fn replies(mut self, replies: Vec<types::Reply>) -> Self {
        self.replies = Some(replies);
        self
    }

    pub fn push_reply(self, reply: types::Reply) -> Self {
        let mut replies = self.replies.clone().unwrap_or_default();
        replies.push(reply);
        self.replies(replies)
    }

    pub fn masquerade(mut self, masquerade: types::Masquerade) -> Self {
        self.masquerade = Some(masquerade);
        self
    }

    pub async fn build(self, http: &HTTPClient) -> types::SendMessage {
        let Self { content, attachments, embeds, replies, masquerade } = self;

        // no async map waaaa
        let attachments = match attachments {
            Some(attachments) => {
                let mut output = Vec::new();

                for file in attachments {
                    let content = utils::read_file(&file).await;

                    let response = http.upload_file("attachments", content, file.name()).await;
                    output.push(response.id);
                };

                output
            },
            None => Vec::new()
        };

        types::SendMessage {
            content,
            attachments,
            embeds: embeds.unwrap_or_default(),
            replies: replies.unwrap_or_default(),
            masquerade
        }
    }
}
