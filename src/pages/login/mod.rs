use dioxus::prelude::*;
use crate::components::input::StateInput;
pub use crate::{println, icons::{Github, Twitter, Mastodon}, styled};

styled!(LoginForm, div, "
    height: 100%;
    width: 100%;
    display: flex;
");

styled!(LoginInner, div, "
    display: flex;
    width: 100%;
    padding: 40px 35px;
    user-select: none;
    background: url('/static/login_background.jpg');
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
");

styled!(LoginPopout, div, "
    flex-grow: 1;
    background-color: rgba(36, 36, 36, 0.75);
    border: 2px solid rgba(128, 128, 128, 0.15);
    backdrop-filter: blur(20px);
    max-width: 360px;
    max-height: 250px;
    padding: 30px 25px;
    border-radius: 8px;
    margin-inline-start: 50px;
    margin-top: 20px;
    margin-bottom: 20px;
    box-shadow: 0 2px 10px 0 rgb(0 0 0 / 20%);
");

styled!(Form, div, "
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 10px;
");

styled!(FormTitle, p, "
    margin: 0 0 -4px;
    font-size: 12px;
    color: #4d5566;
");

styled!(LoginButton, button, "
    background: #0b0e14;
    z-index: 1;
    font-size: 1rem;
    padding: 8px 16px;
    border-radius: var(--border-radius);
    font-family: inherit;
    color: #b3b1ad;
    border: none;
    outline: transparent solid 2px;
");

styled!(LoginTitle, a, "
color: white;
font-size: 35px
");

styled!(LoginSocials, div, "margin-top: 10px;");

pub fn login(cx: Scope) -> Element {
    let email = use_state(&cx, || "".to_string());
    let password = use_state(&cx, || "".to_string());

    cx.render(rsx! (
        LoginForm {
            LoginInner {
                LoginPopout {
                    Form {
                        LoginTitle { "Revolt" },
                        FormTitle { "EMAIL" },
                        StateInput {
                            state: email,
                            name: "email",
                            r#type: "email",
                            placeholder: "Enter your email.",
                        },
                        FormTitle { "PASSWORD" },
                        StateInput {
                            state: password,
                            name: "password",
                            r#type: "password",
                            placeholder: "Enter your password.",
                        }
                        button {
                            class: "login-button",
                            onclick: move |_| println!("{} {}", email.get(), password.get()),
                            "Login"
                        },
                    },
                    LoginSocials {
                        a {
                            href: "https://github.com/revoltchat",
                            target: "_blank",
                            Github {},
                        },
                        a {
                            href: "https://twitter.com/revoltchat",
                            target: "_blank",
                            Twitter {}
                        },
                        a {
                            href: "https://mastodon.social/@revoltchat",
                            target: "_blank",
                            Mastodon {}
                        }
                    }
                }
            }
        }
    ))
}
