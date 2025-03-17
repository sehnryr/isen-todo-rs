use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::model::db::{List, Task, User};
use crate::server::SALT;
use crate::util::password_hash::{hash_password, verify_password};

use super::error::{Error, Result};
use super::pool::Pool;

pub struct Repository {
    pool: Pool,
}

impl Repository {
    pub const fn new() -> Self {
        Self { pool: Pool::new() }
    }
}

// users
impl Repository {
    async fn is_username_available(&mut self, username: &str) -> Result<bool> {
        let pool = self.pool.get_ref().await?;

        let result = sqlx::query!(
            "SELECT username FROM users
            WHERE username = ?
            AND deleted_at IS NULL",
            username
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.is_none())
    }

    pub async fn insert_user(&mut self, username: String, password: String) -> Result<User> {
        if !self.is_username_available(&username).await? {
            return Err(Error::UsernameAlreadyExists);
        }

        let id = Uuid::new_v4();
        let password_hash = hash_password(&password, SALT);

        sqlx::query!(
            "INSERT INTO users (id, username, password_hash)
            VALUES (?, ?, ?)",
            id,
            username,
            password_hash
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(User {
            id,
            username,
            password_hash,
            deleted_at: None,
        })
    }

    pub async fn get_user(&mut self, username: String, password: String) -> Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT
                id as "id: _",
                username,
                password_hash,
                deleted_at as "deleted_at: _"
            FROM users
            WHERE username = ?
            AND deleted_at IS NULL"#,
            username
        )
        .fetch_optional(self.pool.get_ref().await?)
        .await?;

        match user {
            Some(user) if verify_password(&password, &user.password_hash, SALT) => Ok(user),
            Some(_) => Err(Error::InvalidCredentials),
            None => Err(Error::UserNotFound),
        }
    }

    pub async fn delete_user(&mut self, user_id: Uuid) -> Result<()> {
        let deleted_at = Utc::now();

        sqlx::query!(
            "UPDATE users SET deleted_at = ?
            WHERE id = ?
            AND deleted_at IS NULL",
            deleted_at,
            user_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }
}

// lists
impl Repository {
    pub async fn create_list(&mut self, title: String, user_id: Uuid) -> Result<()> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        sqlx::query!(
            "INSERT INTO lists (id, title, created_at, created_by)
            VALUES (?, ?, ?, ?)",
            id,
            title,
            created_at,
            user_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }

    pub async fn get_lists(&mut self, user_id: Uuid) -> Result<Vec<List>> {
        let result = sqlx::query_as!(
            List,
            r#"SELECT
                id as "id: _",
                title,
                created_at as "created_at: _",
                deleted_at as "deleted_at: _"
            FROM lists
            WHERE deleted_at IS NULL
            AND created_by = ?"#,
            user_id
        )
        .fetch_all(self.pool.get_ref().await?)
        .await?;

        Ok(result)
    }

    pub async fn delete_list(&mut self, list_id: Uuid, user_id: Uuid) -> Result<()> {
        let deleted_at = Utc::now();

        sqlx::query!(
            "UPDATE lists SET deleted_at = ?
            WHERE id = ?
            AND deleted_at IS NULL
            AND created_by = ?",
            deleted_at,
            list_id,
            user_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }
}

// lists_users
// impl Repository {
//     pub async fn create_list_user(&mut self, list_id: Uuid, user_id: Uuid) -> Result<()> {
//         sqlx::query!(
//             "INSERT INTO lists_users (list_id, user_id)
//             VALUES (?, ?)
//             ON CONFLICT (list_id, user_id) DO NOTHING",
//             list_id,
//             user_id
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(())
//     }

//     pub async fn get_lists_for_user(&mut self, user_id: Uuid) -> Result<Vec<List>> {
//         let rows = sqlx::query_as!(
//             List,
//             r#"SELECT
//                 id as "id: _",
//                 title,
//                 created_at as "created_at: _",
//                 created_by as "created_by: _",
//                 deleted_at as "deleted_at: _"
//             FROM lists
//             LEFT JOIN lists_users ON lists.id = lists_users.list_id
//             WHERE lists.deleted_at IS NULL
//             AND lists_users.user_id = ?"#,
//             user_id
//         )
//         .fetch_all(self.pool.get_ref().await?)
//         .await?;

//         Ok(rows)
//     }

//     pub async fn delete_list_user(&mut self, list_id: Uuid, user_id: Uuid) -> Result<()> {
//         sqlx::query!(
//             "DELETE FROM lists_users
//             WHERE list_id = ?
//             AND user_id = ?",
//             list_id,
//             user_id
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(())
//     }
// }

// tasks
impl Repository {
    pub async fn create_task(
        &mut self,
        list_id: Uuid,
        title: String,
        due_date: DateTime<Utc>,
        user_id: Uuid,
    ) -> Result<()> {
        let task_id = Uuid::new_v4();
        let created_at = Utc::now();

        sqlx::query!(
            r#"INSERT INTO tasks (id, list_id, title, due_date, created_at, created_by)
            VALUES (?, ?, ?, ?, ?, ?)"#,
            task_id,
            list_id,
            title,
            due_date,
            created_at,
            user_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }

    pub async fn get_list_tasks(&mut self, list_id: Uuid) -> Result<Vec<Task>> {
        let rows = sqlx::query_as!(
            Task,
            r#"SELECT
                id as "id: _",
                list_id as "list_id: _",
                title,
                due_date as "due_date: _",
                created_at as "created_at: _",
                completed_at as "completed_at: _"
            FROM tasks
            WHERE list_id = ?"#,
            list_id
        )
        .fetch_all(self.pool.get_ref().await?)
        .await?;

        Ok(rows)
    }

    pub async fn toggle_task_completion(&mut self, task_id: Uuid, user_id: Uuid) -> Result<()> {
        let completed_at = Utc::now();

        sqlx::query!(
            r#"UPDATE tasks
            SET completed_at = CASE
                WHEN completed_at IS NULL THEN ?
                ELSE NULL
            END,
            completed_by = CASE
                WHEN completed_by IS NULL THEN ?
                ELSE NULL
            END
            WHERE id = ?"#,
            completed_at,
            user_id,
            task_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }
}
