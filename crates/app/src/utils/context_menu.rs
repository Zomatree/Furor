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


use crate::prelude::*;
use std::rc::Rc;

pub enum ActiveContextMenu {
    Message {
        message_id: types::ULID,
        channel_id: types::ULID,
    }
}

#[derive(Clone)]
pub struct UseContextMenu<'a> {
    current: &'a Option<ActiveContextMenu>,
    setter: &'a Rc<dyn Fn(Option<ActiveContextMenu>)>
}

impl<'a> UseContextMenu<'a> {
    pub fn get(&self) -> &'a Option<ActiveContextMenu> {
        self.current
    }

    pub fn set(&self, context_menu: Option<ActiveContextMenu>) {
        (self.setter)(context_menu)
    }
}

pub fn use_context_menu(cx: &ScopeState) -> UseContextMenu<'_> {
    UseContextMenu {
        current: use_read(cx, CONTEXT_MENU),
        setter: use_set(cx, CONTEXT_MENU)
    }
}
