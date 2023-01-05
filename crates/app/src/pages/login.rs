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

#[derive(Clone, Debug)]
enum LoginState {
    Details {
        email: String,
        password: String,
    },
    SelectMfa {
        allowed_methods: Vec<types::MFAMethod>,
        selection: Option<types::MFAMethod>,
        ticket: String,
    },
    InputMfa {
        method: types::MFAMethod,
        ticket: String,
        value: types::MfaResponse
    }
}

impl LoginState {
    pub fn button_text(&self) -> &'static str {
        match self {
            Self::Details  { .. } => "Login",
            Self::InputMfa { .. } | Self::SelectMfa { .. } => "Next"
        }
    }
}

pub fn Login(cx: Scope) -> Element {
    let router = use_router(cx);
    let set_config = use_set(cx, REVOLT_CONFIG);
    let set_user = use_set(cx, USER);
    let login_state = use_state(cx, || LoginState::Details { email: String::new(), password: String::new() });
    let client = cx.use_hook(reqwest::Client::new);

    if get_local_storage_user().is_some() {
        router.push_route("/", None, None)
    }

    let text = login_state.get().button_text();

    cx.render(rsx!(div {
        style: "height: 100%; width: 100%; background-image: url('https://images.unsplash.com/photo-1652195598569-f523c6973b42'); display: flex; flex-direction: column; justify-content: center",
        div {
            style: "border-width: 1px; border-radius: 5px; background-color: rgba(0,0,0,.5); margin-left: 50px; z-index: 10; position: absolute",
            div {
                style: "padding: 20px",
                p {
                    style: "font-weight: 900; margin-top: 0px",
                    "LOGIN"
                },
                div {
                    style: "display: flex; flex-direction: column; justify-content: center; gap: 6px",
                    match login_state.get() {
                        LoginState::Details { .. } => {
                            rsx! {
                                components::Input {
                                    placeholder: "Email",
                                    oninput: |evt: Event<FormData>| login_state.with_mut(|state| {
                                        if let LoginState::Details { email, .. } = state {
                                            *email = evt.value.clone()
                                        };
                                    })
                                },
                                components::Input {
                                    placeholder: "Password",
                                    oninput: |evt: Event<FormData>| login_state.with_mut(|state| {
                                        if let LoginState::Details { password, .. } = state {
                                            *password = evt.value.clone()
                                        };
                                    })
                                },
                            }
                        },
                        LoginState::SelectMfa { allowed_methods, .. } => {
                            rsx! {
                                select {
                                    oninput: move |evt| {
                                        let selection_method = match evt.value.as_str() {
                                            "Password" => types::MFAMethod::Password,
                                            "Recovery Code" => types::MFAMethod::Recovery,
                                            "Totp Code" => types::MFAMethod::Totp,
                                            _ => unreachable!()
                                        };
                                        login_state.with_mut(|state| {
                                            if let LoginState::SelectMfa { selection, .. } = state {
                                                *selection = Some(selection_method);
                                            };
                                        })                                    },
                                    allowed_methods.iter().map(|method| {
                                        rsx!{
                                            option {
                                                key: "{method}",
                                                label: "{method}",
                                                value: "{method}",
                                                "{method}"
                                            }
                                        }
                                    }),
                                }
                            }
                        },
                        LoginState::InputMfa { method, ticket, .. } => {
                            rsx! {
                                components::Input {
                                    oninput: move |evt: Event<FormData>| {
                                        login_state.set(LoginState::InputMfa { method: *method, ticket: ticket.clone(), value: match &method {
                                            types::MFAMethod::Password => types::MfaResponse::Password(evt.value.clone()),
                                            types::MFAMethod::Recovery => types::MfaResponse::RecoveryCode(evt.value.clone()),
                                            types::MFAMethod::Totp => types::MfaResponse::TotpCode(evt.value.clone()),
                                        }})
                                    },
                                    placeholder: "{method}"
                                }
                            }
                        }
                    },
                    components::Button {
                        background: true,
                        onclick: move |_| {
                            match login_state.get() {
                                LoginState::Details { email, password } => {
                                    let set_config = set_config.clone();
                                    let set_user = set_user.clone();
                                    let login_state = login_state.clone();
                                    let client = client.clone();
                                    let email = email.clone();
                                    let password = password.clone();

                                    cx.spawn(async move {
                                        let res = client.get(API_URL)
                                            .send()
                                            .await
                                            .unwrap()
                                            .json::<types::RevoltConfig>()
                                            .await
                                            .unwrap();

                                        set_config(Some(res));

                                        let res = client.post(format!("{API_URL}/auth/session/login"))
                                            .json(&types::LoginBody::Details {
                                                email,
                                                password,
                                                friendly_name: Some(env!("CARGO_PKG_NAME").to_string())
                                            })
                                            .send()
                                            .await
                                            .unwrap()
                                            .json::<types::Login>()
                                            .await
                                            .unwrap();

                                        match res {
                                            types::Login::Success { user_id, token, .. } => {
                                                set_user(Some((types::Token::User(token), types::ULID(user_id))));
                                            },
                                            types::Login::Mfa { allowed_methods, ticket } => {
                                                login_state.set(LoginState::SelectMfa { allowed_methods, selection: None, ticket })
                                            },
                                        }
                                    });

                                },
                                LoginState::SelectMfa { selection, ticket, .. } => {
                                    log::info!("{selection:?}");

                                    if let Some(method) = selection {
                                        login_state.set(LoginState::InputMfa { method: *method, ticket: ticket.clone(), value: match method {
                                            types::MFAMethod::Password => types::MfaResponse::Password(String::new()),
                                            types::MFAMethod::Recovery => types::MfaResponse::RecoveryCode(String::new()),
                                            types::MFAMethod::Totp => types::MfaResponse::TotpCode(String::new()),
                                        } })
                                    }
                                },
                                LoginState::InputMfa { ticket, value, .. } => {
                                    let client = client.clone();
                                    let ticket = ticket.clone();
                                    let value = value.clone();
                                    let set_user = set_user.clone();
                                    let router = router.clone();

                                    cx.spawn(async move {
                                        let res = client.post(format!("{API_URL}/auth/session/login"))
                                            .json(&types::LoginBody::Mfa {
                                                mfa_ticket: ticket,
                                                mfa_response: value,
                                                friendly_name: Some(env!("CARGO_PKG_NAME").to_string())
                                            })
                                            .send()
                                            .await
                                            .unwrap()
                                            .json::<types::Login>()
                                            .await
                                            .unwrap();

                                        if let types::Login::Success { user_id, token, .. } = res {
                                            set_user(Some((types::Token::User(token), types::ULID(user_id))));
                                            router.push_route("/", None, None);
                                        }
                                    });
                                }
                            }
                        },
                        "{text}"
                    }
                }
            }
        }

    }))
}
