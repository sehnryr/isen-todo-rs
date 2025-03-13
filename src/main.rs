use chrono::prelude::*;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _: Signal<Vec<Todo>> = use_context_provider(|| Signal::new(Vec::new()));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // Router::<Route> {}
        TodoList {}
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Todo {
    id: u32,
    title: String,
    due_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
    created_by: String,
    completed: bool,
    completed_at: Option<DateTime<Utc>>,
    completed_by: Option<String>,
}

#[component]
fn TodoList() -> Element {
    let mut todos = use_context::<Signal<Vec<Todo>>>();

    let mut title_value: Signal<String> = use_signal(|| String::new());
    let mut date_value: Signal<Option<DateTime<Utc>>> = use_signal(|| None);

    rsx! {
        ul { id: "todo-list",
            for todo in todos.read().iter() {
                TodoItem { todo: todo.clone() }
            }
        }
        form {
            input {
                r#type: "text",
                required: true,
                value: title_value,
                oninput: move |event| {
                    title_value.set(event.value());
                },
            }
            input {
                r#type: "date",
                required: true,
                value: date_value
                    .read()
                    .as_ref()
                    .map(|date| date.format("%Y-%m-%d").to_string())
                    .unwrap_or_default(),
                oninput: move |event| {
                    let date_str = event.value();
                    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap();
                    date_value.set(Some(date.and_hms_opt(0, 0, 0).unwrap().and_utc()));
                },
            }
            button {
                r#type: "submit",
                onclick: move |event| {
                    event.prevent_default();
                    if title_value.read().trim().is_empty() {
                        return;
                    }
                    if date_value.read().is_none() {
                        return;
                    }
                    let title = title_value.take();
                    let due_date = date_value.take().unwrap();
                    let new_id = todos.read().len() as u32;
                    todos
                        .write()
                        .push(Todo {
                            id: new_id,
                            title: title,
                            due_date: due_date,
                            created_at: Utc::now(),
                            created_by: "".to_string(),
                            completed: false,
                            completed_at: None,
                            completed_by: None,
                        });
                },
                "Add Todo"
            }
        }
    }
}

#[component]
fn TodoItem(todo: Todo) -> Element {
    rsx! {
        li { id: "todo-item",
            h2 { "{todo.title}" }
            p { "{todo.created_at}" }
            if todo.completed {
                p { "{todo.completed_at:?}" }
                p { "{todo.completed_by:?}" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Echo {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div { id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
