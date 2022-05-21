use dioxus::fermi::Atom;
use std::collections::HashMap;

use crate::prelude::*;

pub static USERS: Atom<HashMap<types::ULID, types::User>> = |_| HashMap::new();
pub static SERVERS: Atom<HashMap<types::ULID, types::Server>> = |_| HashMap::new();
pub static CHANNELS: Atom<HashMap<types::ULID, types::Channel>> = |_| HashMap::new();
pub static SERVER_MEMBERS: Atom<HashMap<types::ULID, HashMap<types::ULID, types::Member>>> = |_| HashMap::new();
pub static MESSAGES: Atom<HashMap<types::ULID, HashMap<types::ULID, types::Message>>> = |_| HashMap::new();
