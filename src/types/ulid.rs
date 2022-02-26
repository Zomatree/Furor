use serde::{Deserialize, Serialize};
use crate::{types::{user::User, channel::Channel, server::Server, member::Member}, State};

#[derive(Clone, Eq)]
pub struct ULID(pub String);

impl ULID {
    pub fn to_user<'a>(&self, state: &'a State) -> Option<&'a User> {
        state.users.get(self)
    }

    pub fn to_channel<'a>(&self, state: &'a State) -> Option<&'a Channel> {
        state.channels.get(self)
    }

    pub fn to_server<'a>(&self, state: &'a State) -> Option<&'a Server> {
        state.servers.get(self)
    }

    pub fn to_member<'a>(&self, state: &'a State, server_id: &ULID) -> Option<&'a Member> {
        state.server_members
            .get(server_id)
            .and_then(|members| members.get(self))
    }
}

impl<'de> Deserialize<'de> for ULID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        String::deserialize(deserializer)
            .map(ULID)
    }
}

impl Serialize for ULID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        String::serialize(&self.0, serializer)
    }
}

impl std::ops::Deref for ULID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::hash::Hash for ULID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::cmp::PartialEq for ULID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for ULID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for ULID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
