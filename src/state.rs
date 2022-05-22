use dioxus::fermi::Atom;
use std::rc::Rc;
use im_rc::{HashMap, HashSet};

use crate::prelude::*;

pub type UserCache = HashMap<types::ULID, types::User>;
pub type ServerCache = HashMap<types::ULID, types::Server>;
pub type ChannelCache = HashMap<types::ULID, types::Channel>;
pub type ServerMemberCache =HashMap<types::ULID, HashMap<types::ULID, types::Member>>;
pub type MessageCache = HashMap<types::ULID, HashMap<types::ULID, types::Message>>;
pub type TypingCache = HashMap<types::ULID, HashSet<types::ULID>>;

pub type FermiSetter<T> = Rc<dyn Fn(T)>;

pub static USERS: Atom<UserCache> = |_| HashMap::new();
pub static SERVERS: Atom<ServerCache> = |_| HashMap::new();
pub static CHANNELS: Atom<ChannelCache> = |_| HashMap::new();
pub static SERVER_MEMBERS: Atom<ServerMemberCache> = |_| HashMap::new();
pub static MESSAGES: Atom<MessageCache> = |_| HashMap::new();
pub static TYPING: Atom<TypingCache> = |_| HashMap::new();
