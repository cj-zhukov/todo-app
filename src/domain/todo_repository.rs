use async_trait::async_trait;
use serde::Deserialize;

use crate::{domain::{error::DomainError, todo::Todo}};

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

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<Todo>, DomainError>;

    async fn read_id(&self, id: i64) -> Result<Todo, DomainError>;

    async fn read_body(&self, body: &str) -> Result<Todo, DomainError>;

    async fn create(&self, todo: CreateTodo) -> Result<(), DomainError>;

    async fn update(&self, id: i64, todo: UpdateTodo) -> Result<(), DomainError>;

    async fn delete(&self, id: i64) -> Result<(), DomainError>;
}
