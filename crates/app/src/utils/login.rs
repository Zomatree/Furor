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
