#[cfg(feature = "server")]
use std::sync::{Arc, LazyLock};

use chrono::{DateTime, Utc};
#[cfg(feature = "server")]
use parking_lot::Mutex;
#[cfg(feature = "server")]
use tower_sessions::Session;
use uuid::Uuid;

use dioxus::prelude::*;

use crate::model::db::{List, Task};
// use crate::model::db::Session;
#[cfg(feature = "server")]
use crate::repository::Repository;

#[cfg(feature = "server")]
pub const SALT: &str = "salt";

#[cfg(feature = "server")]
static REPOSITORY: LazyLock<Arc<Mutex<Repository>>> =
    LazyLock::new(|| Arc::new(Mutex::new(Repository::new())));

// #[server]
// pub async fn register_user(email: String, password: String) -> Result<(), ServerFnError> {
//     REPOSITORY.lock().insert_user(email, password).await?;
//     Ok(())
// }

// #[server]
// pub async fn login_user(email: String, password: String) -> Result<Session, ServerFnError> {
//     let session = REPOSITORY.lock().login_user(email, password).await?;
//     Ok(session)
// }

#[server]
pub async fn login(email: String, password: String) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    session.insert("email", email).await?;
    session.insert("is_authenticated", true).await?;

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    session.delete().await?;

    Ok(())
}

#[server]
pub async fn register(email: String, password: String) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    session.insert("email", email).await?;
    session.insert("is_authenticated", true).await?;

    Ok(())
}

#[server]
pub async fn protected() -> Result<String, ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();
    let is_auth: Option<bool> = session.get("is_authenticated").await?;

    if is_auth.unwrap_or(false) {
        let username: Option<String> = session.get("username").await?;
        Ok(format!(
            "Hello, {}! You have access to protected content.",
            username.unwrap_or_else(|| "Guest".into())
        ))
    } else {
        Ok("Unauthorized access".to_string())
    }
}

#[server]
pub async fn create_list(title: String) -> Result<(), ServerFnError> {
    REPOSITORY.lock().create_list(title).await?;
    Ok(())
}

#[server]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    let lists = REPOSITORY.lock().get_lists().await?;
    Ok(lists)
}

#[server]
pub async fn get_list(id: Uuid) -> Result<List, ServerFnError> {
    let list = REPOSITORY.lock().get_list(id).await?;
    Ok(list)
}

#[server]
pub async fn delete_list(id: Uuid) -> Result<(), ServerFnError> {
    REPOSITORY.lock().delete_list(id).await?;
    Ok(())
}

#[server]
pub async fn create_task(
    list_id: Uuid,
    title: String,
    due_date: DateTime<Utc>,
) -> Result<(), ServerFnError> {
    REPOSITORY
        .lock()
        .create_task(list_id, title, due_date)
        .await?;
    Ok(())
}

#[server]
pub async fn get_tasks(id: Uuid) -> Result<Vec<Task>, ServerFnError> {
    let tasks = REPOSITORY.lock().get_list_tasks(id).await?;
    Ok(tasks)
}

#[server]
pub async fn toggle_task_completion(id: Uuid) -> Result<(), ServerFnError> {
    REPOSITORY.lock().toggle_task_completion(id).await?;
    Ok(())
}
