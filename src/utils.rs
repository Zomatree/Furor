use dioxus::core::ScopeState;
use dioxus::fermi::use_read;
use crate::prelude::*;

pub fn get_username_avatar(
    cx: &ScopeState,
    user: &types::User,
    masquerade: &Option<types::Masquerade>,
    channel_id: &types::ULID,
) -> (String, String) {
    match masquerade {
        Some(mask) => (
            mask.name.clone().unwrap_or_else(|| user.username.clone()),
            mask.avatar
                .clone()
                .unwrap_or_else(|| user.avatar.clone().unwrap().url()),
        ),
        None => {
            let channel = use_read(cx, CHANNELS).get(channel_id).unwrap();

            match channel.server() {
                Some(server_id) => {
                    let member = use_read(cx, SERVER_MEMBERS)
                        .get(&server_id)
                        .unwrap()
                        .get(&user.id)
                        .unwrap();

                    let default_avatar = types::Asset::as_default_avatar(user.id.clone());

                    (
                        member
                            .nickname
                            .clone()
                            .unwrap_or_else(|| user.username.clone()),
                        member
                            .avatar
                            .as_ref()
                            .or(user.avatar.as_ref())
                            .unwrap_or(&default_avatar)
                            .url(),
                    )
                }
                None => (
                    user.username.clone(),
                    user.avatar.clone().unwrap().url(),
                ),
            }
        }
    }
}

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

