use sqlx::SqlitePool;

pub(super) struct Pool {
    pool: Option<SqlitePool>,
}

impl Pool {
    pub const fn new() -> Self {
        Self { pool: None }
    }

    async fn connect(&mut self) -> Result<(), sqlx::Error> {
        if self.pool.is_some() {
            return Ok(());
        }

        let pool = SqlitePool::connect("sqlite://todo.db").await?;
        sqlx::migrate!("./migration").run(&pool).await?;
        self.pool = Some(pool);
        Ok(())
    }

    pub async fn get_ref(&mut self) -> Result<&SqlitePool, sqlx::Error> {
        self.connect().await?;
        self.pool.as_ref().ok_or(sqlx::Error::PoolClosed)
    }
}
