use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "server")]
impl List {
    pub fn new(title: String, user_id: Uuid) -> Self {
        List {
            id: Uuid::new_v4(),
            title,
            created_at: Utc::now(),
            created_by: user_id,
            deleted_at: None,
        }
    }
}
