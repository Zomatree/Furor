use gloo::storage::{LocalStorage, Storage};
use im_rc::HashMap;
use crate::prelude::*;

pub fn get_username_avatar(
    channels: &ChannelState,
    server_members: &ServerMemberState,
    revolt_config: &types::RevoltConfig,
    user: &types::User,
    masquerade: &Option<types::Masquerade>,
    channel_id: Option<&types::ULID>,
) -> (String, String) {
    match masquerade {
        Some(mask) => (
            mask.name.clone().unwrap_or_else(|| user.username.clone()),
            mask.avatar
                .clone()
                .unwrap_or_else(|| user.avatar.clone().unwrap().url(&revolt_config.features.autumn.url)),
        ),
        None => {
            let server = channel_id
                .and_then(|id| channels.get(id))
                .and_then(|channel| channel.server());

            let default_avatar = types::Asset::as_default_avatar(user.id.clone());

            match server {
                Some(server_id) => {
                    let member = server_members.get(&server_id)
                        .unwrap()
                        .get(&user.id)
                        .unwrap();


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
                            .url(&revolt_config.features.autumn.url),
                    )
                }
                None => (
                    user.username.clone(),
                    user.avatar.clone().unwrap_or(default_avatar).url(&revolt_config.features.autumn.url),
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

pub fn get_last_channel(server_id: &types::ULID) -> Option<types::ULID> {
    let last_channels = match LocalStorage::get::<HashMap<types::ULID, types::ULID>>("last_channels") {
        Ok(channels) => channels,
        Err(_) => {
            LocalStorage::set("last_channels", HashMap::<types::ULID, types::ULID>::new()).unwrap();
            HashMap::new()
        }
    };

    last_channels
        .get(server_id)
        .cloned()
}

pub fn set_last_channel(server_id: types::ULID, channel_id: types::ULID) {
    let mut last_channels = LocalStorage::get::<HashMap<types::ULID, types::ULID>>("last_channels").unwrap_or_default();

    last_channels.insert(server_id, channel_id);

    LocalStorage::set("last_channels", last_channels).unwrap();
}

pub fn get_local_storage_user() -> Option<(types::Token, types::ULID)> {
    LocalStorage::get::<(types::Token, types::ULID)>("user").ok()
}

pub fn redirect_to_login(cx: &ScopeState) {
    let router = use_router(cx);
    let has_token = get_local_storage_user().is_some();

    if !has_token {
        router.push_route("/login", None, None)
    }
}

pub fn format_datetime<Tz: chrono::TimeZone>(dt: &chrono::DateTime<Tz>) -> String where Tz::Offset: std::fmt::Display {
    let now = chrono::Utc::now();
    let yesterday = now - chrono::Duration::days(1);

    if dt.date() == now.date() {
        dt.format("Today at %H:%M").to_string()
    } else if dt.date() == yesterday.date() {
        dt.format("Yesterday at %H:%M").to_string()
    } else {
        dt.format("%d/%m/%Y").to_string()
    }
}
