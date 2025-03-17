use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    // pub created_by: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    // pub created_by: Uuid,
    pub completed_at: Option<DateTime<Utc>>,
    // pub completed_by: Option<Uuid>,
}

// #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub struct Session {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub created_at: DateTime<Utc>,
//     pub expires_at: DateTime<Utc>,
// }
