use super::db::Todo;

use anyhow::Result;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize, Clone)]
pub struct CreateTodo {
    pub body: String,
}

impl CreateTodo {
    pub fn body(&self) -> &str {
        self.body.as_ref()
    }
}

#[derive(Deserialize, Clone)]
pub struct UpdateTodo {
    body: String,
    completed: bool,
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
    pub async fn create(pool: PgPool, new_todo: CreateTodo) -> Result<Todo> {
        let sql = format!("insert into {} (body) values ($1) returning *", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query
            .bind(new_todo.body())
            .fetch_one(&pool)
            .await?;
        
        Ok(data)
    }

    pub async fn update(pool: PgPool, id: i64, update_todo: UpdateTodo) -> Result<Todo> {
        let sql = format!("
            update {} 
            set body = $1, completed = $2, updated_at = now()::timestamp 
            where id = $3 returning *", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query
            .bind(update_todo.body())
            .bind(update_todo.completed())
            .bind(id)
            .fetch_one(&pool)
            .await?;
        
        Ok(data)
    }

    pub async fn delete(pool: PgPool, id: i64) -> Result<Todo> {
        let sql = format!("delete from {} where id = $1 returning *", Self::table_name());
        let query = sqlx::query_as::<_, Self>(&sql);
        let data = query
            .bind(id)
            .fetch_one(&pool)
            .await?;
        
        Ok(data)
    }
}