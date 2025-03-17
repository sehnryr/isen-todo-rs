use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::model::db::{List, Task};
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
// impl Repository {
//     async fn is_email_available(&mut self, email: &str) -> Result<bool> {
//         let pool = self.pool.get_ref().await?;

//         let result = sqlx::query!(
//             "SELECT email FROM users
//             WHERE email = ?
//             AND deleted_at IS NULL",
//             email
//         )
//         .fetch_optional(pool)
//         .await?;

//         Ok(result.is_none())
//     }

//     pub async fn insert_user(&mut self, email: String, password: String) -> Result<()> {
//         if !self.is_email_available(&email).await? {
//             return Err(Error::EmailAlreadyExists);
//         }

//         let id = Uuid::new_v4();
//         let password_hash = hash_password(&password, SALT);

//         sqlx::query!(
//             "INSERT INTO users (id, email, password_hash)
//             VALUES (?, ?, ?)",
//             id,
//             email,
//             password_hash
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(())
//     }

//     pub async fn get_user(&mut self, email: String, password: String) -> Result<User> {
//         let user = sqlx::query_as!(
//             User,
//             r#"SELECT
//                 id as "id: _",
//                 email,
//                 password_hash,
//                 deleted_at as "deleted_at: _"
//             FROM users
//             WHERE email = ?
//             AND deleted_at IS NULL"#,
//             email
//         )
//         .fetch_optional(self.pool.get_ref().await?)
//         .await?;

//         match user {
//             Some(user) if verify_password(&password, &user.password_hash, SALT) => Ok(user),
//             Some(_) => Err(Error::InvalidCredentials),
//             None => Err(Error::UserNotFound),
//         }
//     }

//     pub async fn update_user_password(
//         &mut self,
//         user_id: Uuid,
//         new_password: String,
//     ) -> Result<()> {
//         let hashed_password = hash_password(&new_password, SALT);

//         sqlx::query!(
//             "UPDATE users SET password_hash = ?
//             WHERE id = ?
//             AND deleted_at IS NULL",
//             hashed_password,
//             user_id
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(())
//     }

//     pub async fn delete_user(&mut self, user_id: Uuid) -> Result<()> {
//         let deleted_at = Utc::now();

//         sqlx::query!(
//             "UPDATE users SET deleted_at = ?
//             WHERE id = ?
//             AND deleted_at IS NULL",
//             deleted_at,
//             user_id
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(())
//     }
// }

// lists
impl Repository {
    pub async fn create_list(&mut self, title: String) -> Result<()> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        sqlx::query!(
            "INSERT INTO lists (id, title, created_at)
            VALUES (?, ?, ?)",
            id,
            title,
            created_at
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }

    pub async fn get_lists(&mut self) -> Result<Vec<List>> {
        let result = sqlx::query_as!(
            List,
            r#"SELECT
                id as "id: _",
                title,
                created_at as "created_at: _",
                deleted_at as "deleted_at: _"
            FROM lists
            WHERE deleted_at IS NULL"#,
        )
        .fetch_all(self.pool.get_ref().await?)
        .await?;

        Ok(result)
    }

    pub async fn get_list(&mut self, list_id: Uuid) -> Result<List> {
        let result = sqlx::query_as!(
            List,
            r#"SELECT
                id as "id: _",
                title,
                created_at as "created_at: _",
                deleted_at as "deleted_at: _"
            FROM lists
            WHERE id = ?
            AND deleted_at IS NULL"#,
            list_id
        )
        .fetch_one(self.pool.get_ref().await?)
        .await?;

        Ok(result)
    }

    pub async fn delete_list(&mut self, list_id: Uuid) -> Result<()> {
        let deleted_at = Utc::now();

        sqlx::query!(
            "UPDATE lists SET deleted_at = ?
            WHERE id = ?
            AND deleted_at IS NULL",
            deleted_at,
            list_id
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
    ) -> Result<()> {
        let task_id = Uuid::new_v4();
        let created_at = Utc::now();

        sqlx::query!(
            r#"INSERT INTO tasks (id, list_id, title, due_date, created_at)
            VALUES (?, ?, ?, ?, ?)"#,
            task_id,
            list_id,
            title,
            due_date,
            created_at
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

    pub async fn toggle_task_completion(&mut self, task_id: Uuid) -> Result<()> {
        let completed_at = Utc::now();

        sqlx::query!(
            r#"UPDATE tasks
            SET completed_at = CASE
                WHEN completed_at IS NULL THEN ?
                ELSE NULL
            END
            WHERE id = ?"#,
            completed_at,
            task_id
        )
        .execute(self.pool.get_ref().await?)
        .await?;

        Ok(())
    }
}

// sessions
// impl Repository {
//     pub async fn login_user(&mut self, email: String, password: String) -> Result<Session> {
//         let user = self.get_user(email, password).await?;

//         let session = Session {
//             id: Uuid::new_v4(),
//             user_id: user.id,
//             created_at: Utc::now(),
//             expires_at: Utc::now() + Duration::days(30),
//         };

//         sqlx::query!(
//             "INSERT INTO sessions (id, user_id, created_at, expires_at)
//             VALUES (?, ?, ?, ?)",
//             session.id,
//             session.user_id,
//             session.created_at,
//             session.expires_at
//         )
//         .execute(self.pool.get_ref().await?)
//         .await?;

//         Ok(session)
//     }

//     pub async fn get_session(&mut self, session_id: Uuid) -> Result<Session> {
//         let session = sqlx::query_as!(
//             Session,
//             r#"SELECT
//                 id as "id: _",
//                 user_id as "user_id: _",
//                 created_at as "created_at: _",
//                 expires_at as "expires_at: _"
//             FROM sessions WHERE id = ?"#,
//             session_id
//         )
//         .fetch_one(self.pool.get_ref().await?)
//         .await?;

//         Ok(session)
//     }

//     pub async fn logout_user(&mut self, session_id: Uuid) -> Result<()> {
//         sqlx::query!("DELETE FROM sessions WHERE id = ?", session_id)
//             .execute(self.pool.get_ref().await?)
//             .await?;

//         Ok(())
//     }
// }
