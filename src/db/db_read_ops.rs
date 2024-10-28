use sqlx::{PgPool, Error};

use crate::utils::constants::test::MAX_ROWS;
use super::db::Todo;

impl Todo {
    pub async fn list(pool: PgPool) -> Result<Vec<Todo>, Error > {
        let sql = format!("select * from {} limit {}", Self::table_name(), MAX_ROWS);
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query.fetch_all(&pool).await?;

        Ok(data)
    }

    pub async fn read(pool: PgPool, id: i64) -> Result<Todo, Error> {
        let sql = format!("select * from {} where id = $1", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query
            .bind(id)
            .fetch_one(&pool)
            .await?;

        Ok(data)
    }
}