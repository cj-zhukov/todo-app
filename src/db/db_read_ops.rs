use color_eyre::eyre::Result;
use sqlx::PgPool;

use crate::utils::constants::test::MAX_ROWS;
use super::db::Todo;
use super::error::TodoStoreError;

impl Todo {
    #[tracing::instrument(name = "Listing todos from PostgreSQL", skip_all)]
    pub async fn list(pool: PgPool) -> Result<Vec<Todo>, TodoStoreError > {
        let sql = format!("select * from {} limit {}", Self::table_name(), MAX_ROWS);
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query
            .fetch_all(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?;

        Ok(data)
    }

    #[tracing::instrument(name = "Reading todo by id from PostgreSQL", skip_all)]
    pub async fn read_id(pool: PgPool, id: i64) -> Result<Todo, TodoStoreError> {
        let sql = format!("select * from {} where id = $1", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        query
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?
            .map(|todo| Ok(todo))
            .ok_or(TodoStoreError::TodoNotFound)?
    }

    #[tracing::instrument(name = "Reading todo by body from PostgreSQL", skip_all)]
    pub async fn read_body(pool: PgPool, body: &str) -> Result<Todo, TodoStoreError> {
        let sql = format!("select * from {} where body = $1", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        query
            .bind(body)
            .fetch_optional(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?
            .map(|todo| Ok(todo))
            .ok_or(TodoStoreError::TodoNotFound)?
    }
}