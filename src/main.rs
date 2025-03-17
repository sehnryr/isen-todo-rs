mod error;
mod model;
#[cfg(feature = "server")]
mod repository;
mod server;
mod util;

use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_sdk::storage::*;
use model::db::Task;
use uuid::Uuid;

use crate::model::db::List;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/lists/:id")]
    Lists { id: Uuid },
    // #[route("/user/login")]
    // Login {},
    // #[route("/user/register")]
    // Register {},
}

fn main() {
    dioxus_sdk::set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // let _session_token =
    //     use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Navbar() -> Element {
    // let session_token =
    //     use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    rsx! {
        nav {
            Link { to: Route::Home {}, "Home" }
                // if session_token.read().is_some() {
        //     button { "Log out" }
        // } else {
        //     Link { to: Route::Login {}, "Login" }
        //     Link { to: Route::Register {}, "Register" }
        // }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    // let session_token =
    //     use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

    // if session_token.read().is_none() {
    //     return rsx! {
    //         div {
    //             p { "log in or register to continue." }
    //         }
    //     };
    // }

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
                    button {
                        onclick: move |_| {
                            async move {
                                server::delete_list(list.id).await.expect("Failed to delete list");
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
                            async move {
                                server::toggle_task_completion(task.id)
                                    .await
                                    .expect("Failed to update task");
                                update_tasks().await;
                            }
                        },
                    }
                    span { "{task.title}" }
                    div {
                        "Due on: "
                        span { {task.due_date.format("%Y-%m-%d").to_string()} }
                    }
                    if let Some(completed_at) = task.completed_at {
                        div {
                            "Completed at: "
                            span { {completed_at.format("%Y-%m-%d %H:%M:%S").to_string()} }
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
            input {
                r#type: "date",
                placeholder: "due date",
                value: "{due_date}",
                oninput: move |event| due_date.set(event.value()),
            }
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    async move {
                        if task_name.read().is_empty() {
                            return;
                        }
                        if due_date.read().is_empty() {
                            return;
                        }
                        let due_date = NaiveDate::parse_from_str(
                                due_date.read().as_str(),
                                "%Y-%m-%d",
                            )
                            .expect("Failed to parse due date");
                        let due_date = due_date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                        server::create_task(id, task_name.read().clone(), due_date)
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

// #[component]
// fn Login() -> Element {
//     rsx! {}
// }

// #[component]
// fn Register() -> Element {
//     let mut session_token =
//         use_synced_storage::<LocalStorage, Option<Session>>("session_token".to_string(), || None);

//     if session_token.read().is_some() {
//         navigator().push(Route::Home {});
//     }

//     let mut email = use_signal(|| String::new());
//     let mut password = use_signal(|| String::new());

//     let mut error_message = use_signal(|| String::new());

//     let handle = move |email: String, password: String| async move {
//         if let Err(err) = server::register_user(email.clone(), password.clone()).await {
//             error_message.set(err.to_string());
//             return;
//         }
//         error_message.set(String::new());

//         let new_session = server::login_user(email, password)
//             .await
//             .expect("Failed to login");
//         session_token.set(Some(new_session));
//         navigator().push(Route::Home {});
//     };

//     rsx! {
//         if !error_message.read().is_empty() {
//             span {
//                 color: "red",
//                 "{error_message}"
//             }
//         }
//         form {
//             input {
//                 name: "email",
//                 r#type: "email",
//                 placeholder: "Email",
//                 oninput: move |event| email.set(event.value()),
//             }
//             input {
//                 name: "password",
//                 r#type: "password",
//                 placeholder: "Password",
//                 oninput: move |event| password.set(event.value()),
//             }
//             button {
//                 onclick: move |event| async move {
//                     event.prevent_default();
//                     handle(email.read().clone(), password.read().clone()).await;
//                 },
//                 "Register"
//             }
//         }
//     }
// }
