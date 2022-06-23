use dioxus::fermi::Atom;
use std::rc::Rc;
use im_rc::{HashMap, HashSet};

use crate::prelude::*;

pub type UserState = HashMap<types::ULID, types::User>;
pub type ServerState = HashMap<types::ULID, types::Server>;
pub type ChannelState = HashMap<types::ULID, types::Channel>;
pub type ServerMemberState =HashMap<types::ULID, HashMap<types::ULID, types::Member>>;
pub type MessageState = HashMap<types::ULID, HashMap<types::ULID, types::Message>>;
pub type TypingState = HashMap<types::ULID, HashSet<types::ULID>>;
pub type DmChannelState = HashSet<types::ULID>;

pub type FermiSetter<T> = Rc<dyn Fn(T)>;

pub static USERS: Atom<UserState> = |_| HashMap::new();
pub static SERVERS: Atom<ServerState> = |_| HashMap::new();
pub static CHANNELS: Atom<ChannelState> = |_| HashMap::new();
pub static SERVER_MEMBERS: Atom<ServerMemberState> = |_| HashMap::new();
pub static MESSAGES: Atom<MessageState> = |_| HashMap::new();
pub static TYPING: Atom<TypingState> = |_| HashMap::new();
pub static DM_CHANNELS: Atom<DmChannelState> = |_| HashSet::new();

pub static CURRENT_SERVER: Atom<Option<types::ULID>> = |_| None;
pub static CURRENT_CHANNEL: Atom<Option<types::ULID>> = |_| None;

pub static REVOLT_CONFIG: Atom<Option<types::RevoltConfig>> = |_| None;

pub static HTTP: Atom<Option<HTTPClient>> = |_| None;

pub static USER: Atom<Option<(types::Token, types::ULID)>> = |_| None;
pub static READY: Atom<bool> = |_| false;
