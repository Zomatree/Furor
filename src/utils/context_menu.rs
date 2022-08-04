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
