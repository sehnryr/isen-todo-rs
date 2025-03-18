use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::model::Task;

use super::error::Result;

impl Task {
    pub async fn create_task(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO tasks (id, list_id, title, due_date, created_at, created_by)
            VALUES (?, ?, ?, ?, ?, ?)"#,
            self.id,
            self.list_id,
            self.title,
            self.due_date,
            self.created_at,
            self.created_by
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_list_tasks(list_id: Uuid, pool: &SqlitePool) -> Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Task,
            r#"SELECT
                tasks.id as "id: _",
                list_id as "list_id: _",
                title,
                due_date as "due_date: _",
                created_at as "created_at: _",
                created_by as "created_by: _",
                completed_at as "completed_at: _",
                users.username as "completed_by: _"
            FROM tasks
            LEFT JOIN users ON tasks.completed_by = users.id
            WHERE list_id = ?"#,
            list_id
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }

    pub async fn complete_task(&self, user_id: Uuid, pool: &SqlitePool) -> Result<()> {
        let completed_at = Utc::now();

        sqlx::query!(
            r#"UPDATE tasks
            SET completed_at = ?,
                completed_by = ?
            WHERE id = ?"#,
            completed_at,
            user_id,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn uncomplete_task(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            r#"UPDATE tasks
            SET completed_at = NULL,
                completed_by = NULL
            WHERE id = ?"#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
