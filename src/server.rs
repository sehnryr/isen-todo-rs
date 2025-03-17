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

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    let user = REPOSITORY.lock().get_user(username, password).await?;

    session.insert("id", user.id).await?;

    Ok(())
}

#[server]
pub async fn register(username: String, password: String) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    let user = REPOSITORY.lock().insert_user(username, password).await?;

    session.insert("id", user.id).await?;

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();

    session.delete().await?;

    Ok(())
}

#[server]
pub async fn create_list(title: String) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();
    let user_id: Uuid = session.get("id").await.unwrap().unwrap();

    REPOSITORY.lock().create_list(title, user_id).await?;
    Ok(())
}

#[server]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();
    let user_id: Uuid = session.get("id").await.unwrap().unwrap();

    let lists = REPOSITORY.lock().get_lists(user_id).await?;
    Ok(lists)
}

#[server]
pub async fn delete_list(id: Uuid) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();
    let user_id: Uuid = session.get("id").await.unwrap().unwrap();

    REPOSITORY.lock().delete_list(id, user_id).await?;
    Ok(())
}

#[server]
pub async fn create_task(
    list_id: Uuid,
    title: String,
    due_date: DateTime<Utc>,
) -> Result<(), ServerFnError> {
    let session = extract::<Session, _>().await.unwrap();
    let user_id: Uuid = session.get("id").await.unwrap().unwrap();

    REPOSITORY
        .lock()
        .create_task(list_id, title, due_date, user_id)
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
    let session = extract::<Session, _>().await.unwrap();
    let user_id: Uuid = session.get("id").await.unwrap().unwrap();

    REPOSITORY
        .lock()
        .toggle_task_completion(id, user_id)
        .await?;
    Ok(())
}
