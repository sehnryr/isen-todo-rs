use sqlx::SqlitePool;

use crate::model::User;

use super::error::{Error, Result};

impl User {
    pub async fn insert_user(&self, pool: &SqlitePool) -> Result<()> {
        if sqlx::query!("SELECT * FROM users WHERE username = ?", self.username)
            .fetch_optional(pool)
            .await?
            .is_some()
        {
            return Err(Error::UserAlreadyExists);
        }

        sqlx::query!(
            "INSERT INTO users (id, username, password_hash)
            VALUES (?, ?, ?)",
            self.id,
            self.username,
            self.password_hash
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_username(username: &str, pool: &SqlitePool) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT
                id as "id: _",
                username,
                password_hash
            FROM users
            WHERE username = ?"#,
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}
