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


use std::rc::Rc;
use crate::prelude::*;


#[derive(Clone)]
pub enum ActiveModal {
    DeleteMessage {
        channel_id: types::ULID,
        message_id: types::ULID
    },
    React {
        channel_id: types::ULID,
        message_id: types::ULID
    }
}

#[derive(Clone)]
pub struct UseModal {
    current: Rc<Vec<ActiveModal>>,
    setter: Rc<dyn Fn(Vec<ActiveModal>)>
}

impl UseModal {
    pub fn push_modal(&self, modal: ActiveModal) {
        let mut current = (*self.current).clone();
        current.push(modal);

        (self.setter)(current)
    }

    pub fn pop_modal(&self) -> Option<ActiveModal> {
        let mut current = (*self.current).clone();
        let last = current.pop();

        (self.setter)(current);

        last
    }
}

pub fn use_modal(cx: &ScopeState) -> UseModal {
    let modals = use_read_rc(cx, MODALS).clone();
    let set_modals = use_set(cx, MODALS).clone();

    UseModal { current: modals, setter: set_modals }
}
