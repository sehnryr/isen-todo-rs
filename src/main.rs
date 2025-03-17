mod error;
mod model;
#[cfg(feature = "server")]
mod repository;
mod server;
mod util;

use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use uuid::Uuid;

use crate::model::db::Session;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Lists { id: Uuid },
    #[route("/user/login")]
    Login {},
    #[route("/user/register")]
    Register {},
}

fn main() {
    dioxus_sdk::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _session_token =
        use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    let session_token =
        use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    rsx! {
        nav {
            Link { to: Route::Home {}, "Home" }
            if session_token.read().is_some() {
                button { "Log out" }
            } else {
                Link { to: Route::Login {}, "Login" }
                Link { to: Route::Register {}, "Register" }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let session_token =
        use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    if session_token.read().is_none() {
        return rsx! {
            div {
                p { "log in or register to continue." }
            }
        };
    }

    rsx! {}
}

#[component]
fn Lists(id: Uuid) -> Element {
    rsx! {}
}

#[component]
fn Login() -> Element {
    rsx! {}
}

#[component]
fn Register() -> Element {
    let mut session_token =
        use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    if session_token.read().is_some() {
        navigator().push(Route::Home {});
    }

    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let mut error_message = use_signal(|| String::new());

    let handle = move |email: String, password: String| async move {
        if let Err(err) = server::register_user(email.clone(), password.clone()).await {
            error_message.set(err.to_string());
            return;
        }
        error_message.set(String::new());

        let new_session = server::login_user(email, password)
            .await
            .expect("Failed to login");
        session_token.set(Some(new_session));
        navigator().push(Route::Home {});
    };

    rsx! {
        if !error_message.read().is_empty() {
            span {
                color: "red",
                "{error_message}"
            }
        }
        form {
            input {
                name: "email",
                r#type: "email",
                placeholder: "Email",
                oninput: move |event| email.set(event.value()),
            }
            input {
                name: "password",
                r#type: "password",
                placeholder: "Password",
                oninput: move |event| password.set(event.value()),
            }
            button {
                onclick: move |event| async move {
                    event.prevent_default();
                    handle(email.read().clone(), password.read().clone()).await;
                },
                "Register"
            }
        }
    }
}
