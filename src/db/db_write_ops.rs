use color_eyre::eyre::Result;
use serde::Deserialize;
use sqlx::PgPool;

use crate::utils::constants::prod::TABLE_NAME;
use super::todo::Todo;
use super::error::TodoStoreError;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateTodo {
    body: String,
}

impl AsRef<str> for CreateTodo {
    fn as_ref(&self) -> &str {
        &self.body
    }
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
}

impl AsRef<str> for UpdateTodo {
    fn as_ref(&self) -> &str {
        &self.body
    }
}

impl UpdateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }

    pub fn completed(&self) -> bool {
        self.completed
    }
}

impl Todo {
    #[tracing::instrument(name = "Creating todo in PostgreSQL", skip_all)]
    pub async fn create(pool: PgPool, new_todo: CreateTodo) -> Result<(), TodoStoreError> {
        let sql = format!("insert into {TABLE_NAME} (body) values ($1)");
        sqlx::query(&sql)
            .bind(new_todo.body())
            .execute(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?;
        
        Ok(())
    }

    #[tracing::instrument(name = "Updating todo in PostgreSQL", skip_all)]
    pub async fn update(pool: PgPool, id: i64, update_todo: UpdateTodo) -> Result<(), TodoStoreError> {
        let sql = format!("
            update {TABLE_NAME} 
            set body = $1, completed = $2, updated_at = now()::timestamp 
            where id = $3");
        sqlx::query(&sql)
            .bind(update_todo.body())
            .bind(update_todo.completed())
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?;
        
        Ok(())
    }

    #[tracing::instrument(name = "Deleting todo in PostgreSQL", skip_all)]
    pub async fn delete(pool: PgPool, id: i64) -> Result<(), TodoStoreError> {
        let sql = format!("delete from {TABLE_NAME} where id = $1");
        sqlx::query(&sql)
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|e| TodoStoreError::UnexpectedError(e.into()))?;
        
        Ok(())
    }
}