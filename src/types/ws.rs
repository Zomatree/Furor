use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::types::{
    message::{Message, Embed},
    user::{User, RelationStatus},
    server::Server,
    channel::Channel,
    asset::Asset,
    permissions::Permissions,
    server::{Category, ServerSystemMessages},
    member::MemberId
};

use super::{role::Role, user::UserStatus, message::MessageEdited, ulid::ULID};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SendWsMessage {
    Authenticate {
        token: String
    },
    BeginTyping {
        channel: String
    },
    EndTyping {
        channel: String
    },
    Ping {
        data: u64
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChannelUpdateData {
    pub name: Option<String>,
    pub recipients: Option<Vec<ULID>>,
    pub description: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChannelUpdateClear {
    Icon,
    Description
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerUpdateData {
    pub owner: Option<ULID>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<Asset>,
    pub banner: Option<Asset>,
    pub default_permissions: Option<Permissions>,
    pub nsfw: Option<bool>,
    pub system_messages: Option<ServerSystemMessages>,
    pub categories: Option<HashMap<String, Category>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerUpdateClear {
    Icon,
    Banner,
    Description
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerMemberUpdateData {
    pub nickname: Option<String>,
    pub avatar: Option<Asset>,
    pub roles: Option<Vec<Role>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerRoleUpdateData {
    pub name: Option<String>,
    pub colour: Option<String>,
    pub hoist: Option<bool>,
    pub rank: Option<i16>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerRoleUpdateClear {
    Colour
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ServerMemberUpdateClear {
    Nickname,
    Avatar
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserUpdateData {
    pub status: Option<UserStatus>,

    #[serde(rename = "profile.background")]
    pub profile_background: Option<Asset>,

    #[serde(rename = "profile.content")]
    pub profile_content: Option<String>,

    pub avatar: Option<Asset>,
    pub online: Option<bool>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum UserUpdateClear {
    ProfileContent,
    ProfileBackground,
    StatusText,
    Avatar
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageUpdateData {
    pub edited: MessageEdited,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReceiveWsMessage {
    Authenticated {},
    Error {
        error: String
    },
    Pong {
        data: u64
    },
    Ready {
        users: Vec<User>,
        servers: Vec<Server>,
        channels: Vec<Channel>
    },
    Message {
        #[serde(flatten)]
        message: Message
    },
    MessageUpdate {
        id: ULID,
        channel: String,

        data: MessageUpdateData
    },
    MessageDelete {
        id: ULID,
        channel: String
    },
    ChannelCreate {
        #[serde(flatten)]
        channel: Channel
    },
    ChannelUpdate {
        id: ULID,
        data: ChannelUpdateData,
        clear: Option<ChannelUpdateClear>
    },
    ChannelDelete {
        id: ULID
    },
    ChannelGroupJoin {
        id: ULID,
        user: ULID
    },
    ChannelGroupLeave {
        id: ULID,
        user: ULID
    },
    ChannelStartTyping {
        id: ULID,
        user: ULID
    },
    ChannelStopTyping {
        id: ULID,
        user: ULID
    },
    ChannelAck {
        id: ULID,
        user: ULID,
        message_id: ULID
    },
    ServerUpdate {
        id: ULID,
        data: ServerUpdateData,
        clear: Option<ServerUpdateClear>
    },
    ServerDelete {
        id: ULID
    },
    ServerMemberUpdate {
        id: MemberId,
        data: ServerMemberUpdateData,
        clear: ServerMemberUpdateClear
    },
    ServerMemberJoin {
        id: ULID,
        user: ULID
    },
    ServerMemberLeave {
        id: ULID,
        user: ULID
    },
    ServerRoleUpdate {
        id: ULID,
        role_id: ULID,
        data: ServerRoleUpdateData,
        clear: ServerRoleUpdateClear
    },
    ServerRoleDelete {
        id: ULID,
        role_id: ULID
    },
    UserUpdate {
        id: ULID,
        data: UserUpdateData,
        clear: Option<UserUpdateClear>
    },
    UserRelationship {
        id: ULID,
        user: ULID,
        status: RelationStatus
    }
}
