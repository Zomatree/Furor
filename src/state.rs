/* Copyright (C) 2022-current  Zomatree <me@zomatree.live>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see https://www.gnu.org/licenses/. */


use fermi::Atom;
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
pub type MessageBuilderState = HashMap<types::ULID, utils::MessageBuilder>;
pub type EmojiState = HashMap<types::ULID, types::Emoji>;

pub type FermiSetter<T> = Rc<dyn Fn(T)>;

pub static USERS: Atom<UserState> = |_| HashMap::new();
pub static SERVERS: Atom<ServerState> = |_| HashMap::new();
pub static CHANNELS: Atom<ChannelState> = |_| HashMap::new();
pub static SERVER_MEMBERS: Atom<ServerMemberState> = |_| HashMap::new();
pub static MESSAGES: Atom<MessageState> = |_| HashMap::new();
pub static TYPING: Atom<TypingState> = |_| HashMap::new();
pub static DM_CHANNELS: Atom<DmChannelState> = |_| HashSet::new();
pub static EMOJIS: Atom<EmojiState> = |_| HashMap::new();

pub static CURRENT_SERVER: Atom<Option<types::ULID>> = |_| None;
pub static CURRENT_CHANNEL: Atom<Option<types::ULID>> = |_| None;
pub static CURRENTLY_EDITING: Atom<Option<types::ULID>> = |_| None;

pub static REVOLT_CONFIG: Atom<Option<types::RevoltConfig>> = |_| None;

pub static HTTP: Atom<Option<HTTPClient>> = |_| None;

pub static USER: Atom<Option<(types::Token, types::ULID)>> = |_| None;
pub static READY: Atom<bool> = |_| false;
pub static SAVED_MESSAGES: Atom<Option<types::SavedMessages>> = |_| None;
pub static MODALS: Atom<Vec<utils::ActiveModal>> = |_| Vec::new();
pub static MESSAGE_BUILDERS: Atom<HashMap<types::ULID, utils::MessageBuilder>> = |_| HashMap::new();
pub static CONTEXT_MENU: Atom<Option<ActiveContextMenu>> = |_| None;
