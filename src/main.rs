mod model;
#[cfg(feature = "server")]
mod repository;
mod server;
mod util;

use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use uuid::Uuid;

use crate::model::{List, Task};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/lists/:id")]
    Lists { id: Uuid },
    #[route("/user/login")]
    Login {},
    #[route("/user/register")]
    Register {},
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus_sdk::set_dir!();
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use std::any::Any;
    use std::sync::Arc;

    use axum::Router;
    use sqlx::SqlitePool;
    use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

    let pool = SqlitePool::connect("sqlite://todo.db").await.unwrap();
    sqlx::migrate!("./migration").run(&pool).await.unwrap();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnSessionEnd);

    let pool_provider = move || Box::new(pool.clone()) as Box<dyn Any>;

    let config =
        ServeConfigBuilder::default().context_providers(Arc::new(vec![Box::new(pool_provider)]));

    let router = Router::new()
        .serve_dioxus_application(config, App)
        .layer(session_layer)
        .into_make_service();

    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind to socket address");

    axum::serve(listener, router)
        .await
        .expect("Failed to serve");
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(false));

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    let mut auth = use_context::<Signal<bool>>();

    rsx! {
        nav {
            Link { to: Route::Home {}, "Home" }
            if auth.read().clone() {
                " "
                button {
                    onclick: move |_| async move {
                        server::logout().await.expect("Failed to logout");
                        auth.set(false);
                        navigator().push(Route::Home {});
                    },
                    "Log out"
                }
            } else {
                " "
                Link { to: Route::Login {}, "Login" }
                " "
                Link { to: Route::Register {}, "Register" }
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let auth = use_context::<Signal<bool>>();

    if !auth.read().clone() {
        return rsx! {
            div {
                h1 { "Welcome!" }
                p { "Please log in or register to continue." }
            }
        };
    }

    let mut lists =
        use_synced_storage::<LocalStorage, Vec<List>>("lists".to_string(), || Vec::new());

    let mut list_name = use_signal(|| String::new());

    let update_lists = move || async move {
        lists.set(server::get_lists().await.expect("Failed to get lists"));
    };

    use_future(update_lists);

    rsx! {
        ul {
            for list in lists.read().clone() {
                li {
                    Link { to: Route::Lists { id: list.id }, "{list.title}" }
                    " "
                    button {
                        onclick: move |_| {
                            let list_clone = list.clone();
                            async move {
                                server::delete_list(list_clone).await.expect("Failed to delete list");
                                update_lists().await;
                            }
                        },
                        "Delete"
                    }
                }
            }
        }
        form {
            input {
                r#type: "text",
                placeholder: "list name",
                value: "{list_name}",
                oninput: move |event| list_name.set(event.value()),
            }
            " "
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    async move {
                        server::create_list(list_name.read().clone())
                            .await
                            .expect("Failed to create list");
                        update_lists().await;
                    }
                },
                "Create"
            }
        }
    }
}

#[component]
fn Lists(id: Uuid) -> Element {
    let auth = use_context::<Signal<bool>>();

    if !auth.read().clone() {
        navigator().push(Route::Home {});
        return rsx! {};
    }

    let mut tasks =
        use_synced_storage::<LocalStorage, Vec<Task>>(format!("tasks_{}", id), || Vec::new());

    let mut task_name = use_signal(|| String::new());
    let mut due_date = use_signal(|| String::new());

    let update_tasks = move || async move {
        tasks.set(server::get_tasks(id).await.expect("Failed to get tasks"));
    };

    use_future(update_tasks);

    rsx! {
        ul {
            for task in tasks.read().clone() {
                li {
                    input {
                        r#type: "checkbox",
                        checked: "{task.completed_at.is_some()}",
                        onchange: move |_| {
                            let task_clone = task.clone();
                            async move {
                                if task_clone.completed_at.is_none() {
                                    server::complete_task(task_clone)
                                        .await
                                        .expect("Failed to complete task");
                                } else {
                                    server::uncomplete_task(task_clone)
                                        .await
                                        .expect("Failed to uncomplete task");
                                }
                                update_tasks().await;
                            }
                        },
                    }
                    " "
                    span { "{task.title}" }
                    div {
                        "Due on: "
                        span { {task.due_date.format("%Y-%m-%d").to_string()} }
                    }
                    if let (Some(completed_at), Some(completed_by)) = (
                        task.completed_at,
                        task.completed_by.as_ref(),
                    )
                    {
                        div {
                            "Completed at: "
                            span { {completed_at.format("%Y-%m-%d %H:%M:%S").to_string()} }
                            " by "
                            span { {completed_by.to_string()} }
                        }
                    }
                }
            }
        }
        form {
            input {
                r#type: "text",
                placeholder: "task name",
                value: "{task_name}",
                oninput: move |event| task_name.set(event.value()),
            }
            " "
            input {
                r#type: "date",
                placeholder: "due date",
                value: "{due_date}",
                oninput: move |event| due_date.set(event.value()),
            }
            " "
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    async move {
                        let task_name = task_name.read().clone();
                        let due_date = due_date.read().clone();
                        if task_name.is_empty() {
                            return;
                        }
                        if due_date.is_empty() {
                            return;
                        }
                        let due_date = NaiveDate::parse_from_str(&due_date, "%Y-%m-%d")
                            .expect("Failed to parse due date");
                        let due_date = due_date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                        server::create_task(task_name, due_date, id)
                            .await
                            .expect("Failed to create list");
                        update_tasks().await;
                    }
                },
                "Create"
            }
        }
    }
}

#[component]
fn Login() -> Element {
    let mut auth = use_context::<Signal<bool>>();

    if auth.read().clone() {
        navigator().push(Route::Home {});
        return rsx! {};
    }

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let mut message = use_signal(|| String::new());

    rsx! {
        if !message.read().is_empty() {
            p { "{message}" }
        }
        form {
            input {
                r#type: "text",
                placeholder: "Username",
                value: username.read().clone(),
                oninput: move |event| username.set(event.value()),
            }
            " "
            input {
                r#type: "password",
                placeholder: "Password",
                value: password.read().clone(),
                oninput: move |event| password.set(event.value()),
            }
            " "
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    async move {
                        let username = username.read().clone();
                        let password = password.read().clone();
                        if let Err(_) = server::login(username, password).await {
                            message.set("Login failed".to_owned());
                        } else {
                            auth.set(true);
                            navigator().push(Route::Home {});
                        }
                    }
                },
                "Login"
            }
        }
    }
}

#[component]
fn Register() -> Element {
    let mut auth = use_context::<Signal<bool>>();

    if auth.read().clone() {
        navigator().push(Route::Home {});
        return rsx! {};
    }

    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let mut message = use_signal(|| String::new());

    rsx! {
        if !message.read().is_empty() {
            p { "{message}" }
        }
        form {
            input {
                r#type: "text",
                placeholder: "Username",
                value: username.read().clone(),
                oninput: move |event| username.set(event.value()),
            }
            " "
            input {
                r#type: "password",
                placeholder: "Password",
                value: password.read().clone(),
                oninput: move |event| password.set(event.value()),
            }
            " "
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    async move {
                        let username = username.read().clone();
                        let password = password.read().clone();
                        if let Err(_) = server::register(username, password).await {
                            message.set("Registration failed".to_owned());
                        } else {
                            auth.set(true);
                            navigator().push(Route::Home {});
                        }
                    }
                },
                "Register"
            }
        }
    }
}
