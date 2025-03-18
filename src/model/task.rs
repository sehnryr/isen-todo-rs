use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub list_id: Uuid,
    pub title: String,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<String>,
}

#[cfg(feature = "server")]
impl Task {
    pub fn new(title: String, due_date: DateTime<Utc>, list_id: Uuid, user_id: Uuid) -> Self {
        Task {
            id: Uuid::new_v4(),
            list_id,
            title,
            due_date,
            created_at: Utc::now(),
            created_by: user_id,
            completed_at: None,
            completed_by: None,
        }
    }
}
