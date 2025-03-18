use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::password_hash::{hash_password, verify_password};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(default, skip_serializing)]
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            password_hash: hash_password(&password),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify_password(password, &self.password_hash)
    }
}
