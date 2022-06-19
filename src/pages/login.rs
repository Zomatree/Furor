use crate::prelude::*;
use std::sync::Mutex;

#[derive(Clone, Debug)]
enum LoginState {
    Details {
        email: String,
        password: String,
    },
    Captcha {
        email: String,
        password: String,

        result: Option<String>
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
            Self::Details  { .. } | Self::Captcha { .. } | Self::SelectMfa { .. } => "Login",
            Self::InputMfa { .. } => "Next"
        }
    }
}

pub fn Login(cx: Scope) -> Element {
    let router = use_router(&cx);
    let set_config = use_set(&cx, REVOLT_CONFIG);
    let config = use_read(&cx, REVOLT_CONFIG);
    let set_user = use_set(&cx, USER);
    let login_state = use_state(&cx, || LoginState::Details { email: String::new(), password: String::new() });
    let client = cx.use_hook(|_| reqwest::Client::new());

    if get_local_storage_user().is_some() {
        router.push_route("/", None, None)
    }

    rsx!(cx, div {
        style: "display: flex; align-content: center",
        div {
            style: "display: flex; justify-content: center",
            match login_state.get() {
                LoginState::Details { .. } => {
                    rsx! {
                        input {
                            placeholder: "Email",
                            oninput: |evt| login_state.with_mut(|state| {
                                if let LoginState::Details { email, .. } = state {
                                    *email = evt.value.clone()
                                };
                            })
                        },
                        input {
                            placeholder: "Password",
                            oninput: |evt| login_state.with_mut(|state| {
                                if let LoginState::Details { password, .. } = state {
                                    *password = evt.value.clone()
                                };
                            })
                        },
                    }
                },
                LoginState::Captcha { .. } => {
                    let sitekey = config.as_ref().unwrap().features.captcha.key.clone();
                    let login_state = login_state.clone();

                    rsx! {
                        components::HCaptcha {
                            sitekey: sitekey,
                            complete_callback: Mutex::new(Some(move |token: String| {
                                login_state.with_mut(|state| {
                                    if let LoginState::Captcha { result, .. } = state {
                                        *result = Some(token)
                                    };
                                });
                            }))
                        }
                    }
                },
                LoginState::SelectMfa { allowed_methods, ticket, .. } => {
                    rsx! {
                        select {
                            oninput: move |evt| {
                                let selection = match evt.value.as_str() {
                                    "Password" => types::MFAMethod::Password,
                                    "Recovery Code" => types::MFAMethod::Recovery,
                                    "Totp Code" => types::MFAMethod::Totp,
                                    _ => unreachable!()
                                };
                                login_state.set(LoginState::SelectMfa { allowed_methods: allowed_methods.clone(), selection: Some(selection), ticket: ticket.clone() })
                            },
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
                        input {
                            oninput: move |evt| {
                                login_state.set(LoginState::InputMfa { method: method.clone(), ticket: ticket.clone(), value: match &method {
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
            button {
                onclick: move |_| {
                    match login_state.get() {
                        LoginState::Details { email, password } => {
                            let set_config = set_config.clone();
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

                                login_state.set(LoginState::Captcha {
                                    email: email.clone(),
                                    password: password.clone(),
                                    result: None
                                });
                            });

                        },
                        LoginState::Captcha { email, password, result } => {
                            let email = email.clone();
                            let password = password.clone();
                            let result = result.clone();
                            let set_user = set_user.clone();
                            let login_state = login_state.clone();
                            let client = client.clone();

                            cx.spawn(async move {
                                let res = client.post(format!("{API_URL}/auth/session/login"))
                                    .json(&types::LoginBody::Details {
                                        email,
                                        password,
                                        captcha: result,
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
                                    types::Login::Mfa { ticket, allowed_methods } => {
                                        login_state.set(LoginState::SelectMfa { allowed_methods, ticket, selection: None })
                                    },
                                }
                            });
                        },
                        LoginState::SelectMfa { selection, ticket, .. } => {
                            if let Some(method) = selection {
                                login_state.set(LoginState::InputMfa { method: method.clone(), ticket: ticket.clone(), value: match method {
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
                [format_args!("{}", login_state.get().button_text())]
            }
        }
    })
}
