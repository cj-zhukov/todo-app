use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::PgPool;

use crate::domain::error::DomainError;
use crate::domain::todo::Todo;
use crate::domain::todo_repository::*;
use crate::utils::constants::prod::*;

pub struct PgTodoRepository {
    pool: PgPool,
}

impl PgTodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
pub struct TodoRow {
    pub id: i64,
    pub body: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<TodoRow> for Todo {
    fn from(row: TodoRow) -> Self {
        Self {
            id: row.id,
            body: row.body,
            completed: row.completed,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[async_trait]
impl TodoRepository for PgTodoRepository {
    #[tracing::instrument(name = "Listing todos from PostgreSQL", skip_all)]
    async fn list(&self) -> Result<Vec<Todo>, DomainError> {
        let sql = format!("select * from {TABLE_NAME} limit {MAX_ROWS}");
        let query = sqlx::query_as::<_, TodoRow>(&sql);
        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?;
        let data = rows.into_iter().map(Todo::from).collect();
        Ok(data)
    }

    #[tracing::instrument(name = "Reading todo by id from PostgreSQL", skip_all)]
    async fn read_id(&self, id: i64) -> Result<Todo, DomainError> {
        let sql = format!("select * from {TABLE_NAME} where id = $1");
        let query = sqlx::query_as::<_, TodoRow>(&sql);
        query
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?
            .map(Todo::from) 
            .ok_or(DomainError::TodoNotFound.into())
    }

    #[tracing::instrument(name = "Reading todo by body from PostgreSQL", skip_all)]
    async fn read_body(&self, body: &str) -> Result<Todo, DomainError> {
        let sql = format!("select * from {TABLE_NAME} where body = $1");
        let query = sqlx::query_as::<_, TodoRow>(&sql);
        query
            .bind(body)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?
            .map(Todo::from) 
            .ok_or(DomainError::TodoNotFound.into())
    }

    #[tracing::instrument(name = "Creating todo in PostgreSQL", skip_all)]
    async fn create(&self, todo: CreateTodo) -> Result<(), DomainError> {
        let sql = format!("insert into {TABLE_NAME} (body) values ($1)");
        if self.read_body(todo.body()).await.is_ok() {
            return Err(DomainError::TodoAlreadyExists.into());
        }
        sqlx::query(&sql)
            .bind(todo.body())
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?;
        Ok(())
    }

    #[tracing::instrument(name = "Updating todo in PostgreSQL", skip_all)]
    async fn update(&self, id: i64, todo: UpdateTodo) -> Result<(), DomainError> {
        let sql = format!("
            update {TABLE_NAME} 
            set body = $1, completed = $2, updated_at = now()::timestamp 
            where id = $3");
        sqlx::query(&sql)
            .bind(todo.body())
            .bind(todo.completed())
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?;
        Ok(())
    }

    #[tracing::instrument(name = "Deleting todo in PostgreSQL", skip_all)]
    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let sql = format!("delete from {TABLE_NAME} where id = $1");
        sqlx::query(&sql)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::UnexpectedError(e.into()))?;
        Ok(())
    }
}

