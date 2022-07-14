use futures::{Future};
use std::{pin::Pin, sync::Mutex, rc::Rc};

use crate::prelude::*;

pub type BoxFuture<O> = Pin<Box<dyn Future<Output=O> + 'static>>;
pub type Take<T> = Mutex<Option<T>>;
pub type TakenAsyncFunc = Take<Box<dyn FnOnce() -> BoxFuture<()>>>;

pub fn wrap_async<F, Fut>(func: F) -> TakenAsyncFunc
where
    F: FnOnce() -> Fut + 'static,
    Fut: Future<Output=()> + 'static
{
    Mutex::new(Some(Box::new(|| Box::pin(func()))))
}

#[derive(Clone)]
pub enum ActiveModal {
    DeleteMessage {
        channel_id: types::ULID,
        message_id: types::ULID
    }
}

#[derive(Clone)]
pub struct UseModal<'a> {
    current: &'a Vec<ActiveModal>,
    setter: &'a Rc<dyn Fn(Vec<ActiveModal>)>
}

impl<'a> UseModal<'a> {
    pub fn push_modal(&self, modal: ActiveModal) {
        let mut current = self.current.clone();
        current.push(modal);

        (self.setter)(current)
    }

    pub fn pop_modal(&self) -> Option<ActiveModal> {
        let mut current = self.current.clone();
        let last = current.pop();

        (self.setter)(current);

        last
    }
}

pub fn use_modal(cx: &ScopeState) -> UseModal<'_> {
    let modals = use_read(cx, MODALS);
    let set_modals = use_set(cx, MODALS);

    UseModal { current: modals, setter: set_modals }
}
