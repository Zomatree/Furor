use std::rc::Rc;
use crate::prelude::*;


#[derive(Clone)]
pub enum ActiveModal {
    DeleteMessage {
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
