use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::types::{
    message::{Message, Embed},
    user::{User, RelationStatus},
    server::Server,
    channel::Channel,
    asset::Asset,
    server::{Category, ServerSystemMessages},
    member::MemberId
};

use super::{role::Role, user::UserStatus, ulid::ULID};

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
    pub default_permissions: Option<u64>,
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
    pub edited: String,
    pub content: Option<String>,
    pub embeds: Option<Vec<Embed>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageAppendData {
    #[serde(default)]
    embeds: Option<Vec<Embed>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ReceiveWsMessage {
    Authenticated,
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
        #[serde(rename="id")] message_id: ULID,
        #[serde(rename="channel")] channel_id: ULID,

        data: MessageUpdateData
    },
    MessageDelete {
        #[serde(rename="id")] message_id: ULID,
        #[serde(rename="channel")] channel_id: ULID
    },
    MessageAppend {
        #[serde(rename="id")] message_id: ULID,
        #[serde(rename="channel")] channel_id: ULID,
        append: MessageAppendData
    },
    ChannelCreate {
        #[serde(flatten)]
        channel: Channel
    },
    ChannelUpdate {
        #[serde(rename="id")] channel_id: ULID,
        data: ChannelUpdateData,
        clear: Vec<ChannelUpdateClear>
    },
    ChannelDelete {
        #[serde(rename="id")] channel_id: ULID
    },
    ChannelGroupJoin {
        #[serde(rename="id")] channel_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ChannelGroupLeave {
        #[serde(rename="id")] channel_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ChannelStartTyping {
        #[serde(rename="id")] channel_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ChannelStopTyping {
        #[serde(rename="id")] channel_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ChannelAck {
        #[serde(rename="id")] channel_id: ULID,
        #[serde(rename="user")] user_id: ULID,
        message_id: ULID
    },
    ServerUpdate {
        #[serde(rename="id")] server_id: ULID,
        data: ServerUpdateData,
        clear: Vec<ServerUpdateClear>
    },
    ServerDelete {
        #[serde(rename="id")] server_id: ULID
    },
    ServerMemberUpdate {
        #[serde(rename="id")] member_id: MemberId,
        data: ServerMemberUpdateData,
        clear: Vec<ServerMemberUpdateClear>
    },
    ServerMemberJoin {
        #[serde(rename="id")] server_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ServerMemberLeave {
        #[serde(rename="id")] server_id: ULID,
        #[serde(rename="user")] user_id: ULID
    },
    ServerRoleUpdate {
        #[serde(rename="id")] server_id: ULID,
        role_id: ULID,
        data: ServerRoleUpdateData,
        clear: Vec<ServerRoleUpdateClear>
    },
    ServerRoleDelete {
        #[serde(rename="id")] server_id: ULID,
        role_id: ULID
    },
    UserUpdate {
        #[serde(rename="id")] user_id: ULID,
        data: UserUpdateData,
        clear: Vec<UserUpdateClear>
    },
    UserRelationship {
        id: ULID,
        #[serde(rename="user")] user_id: ULID,
        status: RelationStatus
    }
}
