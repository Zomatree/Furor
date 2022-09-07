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


#[allow(clippy::module_inception)]
pub mod message;
pub mod message_area;
pub mod message_editor;
pub mod message_reactions;

pub use message::Message;
pub use message_area::MessageArea;
pub use message_editor::MessageEditor;
pub use message_reactions::MessageReactions;
