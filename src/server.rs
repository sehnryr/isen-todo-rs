#[cfg(feature = "server")]
use parking_lot::Mutex;
#[cfg(feature = "server")]
use std::sync::{Arc, LazyLock};
use uuid::Uuid;

use dioxus::prelude::*;

use crate::model::db::List;
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
pub async fn create_list(name: String) -> Result<(), ServerFnError> {
    REPOSITORY.lock().create_list(name).await?;
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
