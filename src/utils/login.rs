use gloo::storage::{LocalStorage, Storage};
use crate::prelude::*;


pub fn get_local_storage_user() -> Option<(types::Token, types::ULID)> {
    LocalStorage::get::<(types::Token, types::ULID)>("user").ok()
}

pub fn redirect_to_login(cx: &ScopeState) {
    let router = use_router(cx);
    let has_token = get_local_storage_user().is_some();

    if !has_token {
        router.push_route("/login", None, None)
    }
}
