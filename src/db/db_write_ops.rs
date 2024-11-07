use serde::Deserialize;
use sqlx::{PgPool, Error};

use super::db::Todo;

#[derive(Deserialize, Clone)]
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

#[derive(Deserialize, Clone)]
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
    pub async fn create(pool: PgPool, new_todo: CreateTodo) -> Result<(), Error> {
        let sql = format!("insert into {} (body) values ($1) returning *", Self::table_name());
        sqlx::query(&sql)
            .bind(new_todo.body())
            .execute(&pool)
            .await?;
        
        Ok(())
    }

    pub async fn update(pool: PgPool, id: i64, update_todo: UpdateTodo) -> Result<(), Error> {
        let sql = format!("
            update {} 
            set body = $1, completed = $2, updated_at = now()::timestamp 
            where id = $3", Self::table_name());
        sqlx::query(&sql)
            .bind(update_todo.body())
            .bind(update_todo.completed())
            .bind(id)
            .execute(&pool)
            .await?;
        
        Ok(())
    }

    pub async fn delete(pool: PgPool, id: i64) -> Result<(), Error> {
        let sql = format!("delete from {} where id = $1", Self::table_name());
        sqlx::query(&sql)
            .bind(id)
            .execute(&pool)
            .await?;
        
        Ok(())
    }
}