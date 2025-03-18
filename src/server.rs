use chrono::{DateTime, Utc};
use dioxus::prelude::*;
#[cfg(feature = "server")]
use sqlx::SqlitePool;
#[cfg(feature = "server")]
use tower_sessions::Session;
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::model::User;
use crate::model::{List, Task};

#[cfg(feature = "server")]
#[doc(hidden)]
macro_rules! session {
    () => {
        extract::<Session, _>()
    };
}

#[cfg(feature = "server")]
#[doc(hidden)]
macro_rules! pool {
    () => {
        async { extract().await.map(|FromContext::<SqlitePool>(pool)| pool) }
    };
}

#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    let pool = pool!().await?;
    let session = session!().await.unwrap();

    let user = match User::get_user_by_username(&username, &pool).await? {
        Some(user) => user,
        None => return Err(ServerFnError::new("User not found")),
    };

    if !user.verify_password(&password) {
        return Err(ServerFnError::new("Invalid password"));
    }

    session.insert("user", user).await?;

    Ok(())
}

#[server]
pub async fn register(username: String, password: String) -> Result<(), ServerFnError> {
    let pool = pool!().await?;
    let session = session!().await.unwrap();

    let user = User::new(username, password);
    user.insert_user(&pool).await?;
    session.insert("user", user).await?;

    Ok(())
}

#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let session = session!().await.unwrap();
    session.delete().await?;
    Ok(())
}

#[server]
pub async fn create_list(title: String) -> Result<(), ServerFnError> {
    let session = session!().await.unwrap();
    let pool = pool!().await?;

    let user: User = session.get("user").await.unwrap().unwrap();
    let list = List::new(title, user.id);

    list.create_list(&pool).await?;
    Ok(())
}

#[server]
pub async fn get_lists() -> Result<Vec<List>, ServerFnError> {
    let pool = pool!().await?;
    let session = session!().await.unwrap();

    let user: User = session.get("user").await.unwrap().unwrap();
    let lists = List::get_user_lists(user.id, &pool).await?;

    Ok(lists)
}

#[server]
pub async fn delete_list(list: List) -> Result<(), ServerFnError> {
    let pool = pool!().await?;
    list.delete_list(&pool).await?;
    Ok(())
}

#[server]
pub async fn create_task(
    title: String,
    due_date: DateTime<Utc>,
    list_id: Uuid,
) -> Result<(), ServerFnError> {
    let session = session!().await.unwrap();
    let pool = pool!().await?;

    let user: User = session.get("user").await.unwrap().unwrap();
    let task = Task::new(title, due_date, list_id, user.id);

    task.create_task(&pool).await?;
    Ok(())
}

#[server]
pub async fn get_tasks(list_id: Uuid) -> Result<Vec<Task>, ServerFnError> {
    let pool = pool!().await?;
    let tasks = Task::get_list_tasks(list_id, &pool).await?;
    Ok(tasks)
}

#[server]
pub async fn complete_task(task: Task) -> Result<(), ServerFnError> {
    let session = session!().await.unwrap();
    let pool = pool!().await?;

    let user: User = session.get("user").await.unwrap().unwrap();
    task.complete_task(user.id, &pool).await?;
    Ok(())
}

#[server]
pub async fn uncomplete_task(task: Task) -> Result<(), ServerFnError> {
    let pool = pool!().await?;
    task.uncomplete_task(&pool).await?;
    Ok(())
}
