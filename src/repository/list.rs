use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::model::List;

use super::error::Result;

impl List {
    pub async fn create_list(&self, pool: &SqlitePool) -> Result<()> {
        sqlx::query!(
            "INSERT INTO lists (id, title, created_at, created_by)
            VALUES (?, ?, ?, ?)",
            self.id,
            self.title,
            self.created_at,
            self.created_by
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_lists(user_id: Uuid, pool: &SqlitePool) -> Result<Vec<List>> {
        let result = sqlx::query_as!(
            List,
            r#"SELECT
                id as "id: _",
                title,
                created_at as "created_at: _",
                created_by as "created_by: _",
                deleted_at as "deleted_at: _"
            FROM lists
            WHERE deleted_at IS NULL
            AND created_by = ?"#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_list(&self, pool: &SqlitePool) -> Result<()> {
        let deleted_at = Utc::now();

        sqlx::query!(
            "UPDATE lists SET deleted_at = ?
            WHERE id = ?
            AND deleted_at IS NULL",
            deleted_at,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
