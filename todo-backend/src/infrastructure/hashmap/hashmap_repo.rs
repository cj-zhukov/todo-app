use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::RwLock;

use crate::domain::error::DomainError;
use crate::domain::todo::Todo;
use crate::domain::todo_repository::*;

pub struct HmTodoRepository {
    hm: Arc<RwLock<HashMap<i64, Todo>>>
}

impl HmTodoRepository {
    pub fn new() -> Self {
        Self {
            hm: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl TodoRepository for HmTodoRepository {
    #[tracing::instrument(name = "Listing todos from hashmap", skip_all)]
    async fn list(&self) -> Result<Vec<Todo>, DomainError> {
        let hm = self.hm.read().await;
        Ok(hm.values().cloned().collect())
    }

    #[tracing::instrument(name = "Reading todo by id from hashmap", skip_all)]
    async fn read_id(&self, id: i64) -> Result<Todo, DomainError> {
        let hm = self.hm.read().await;
        hm.get(&id)
            .cloned()
            .ok_or(DomainError::TodoNotFound)
    }

    #[tracing::instrument(name = "Reading todo by body from hashmap", skip_all)]
    async fn read_body(&self, body: &str) -> Result<Todo, DomainError> {
        let hm = self.hm.read().await;
        hm.values()
            .find(|todo| todo.body == body)
            .cloned()
            .ok_or(DomainError::TodoNotFound)
    }

    #[tracing::instrument(name = "Creating todo in hashmap", skip_all)]
    async fn create(&self, todo: CreateTodo) -> Result<(), DomainError> {
        let mut hm = self.hm.write().await;
        if hm.values().any(|t| t.body == todo.body()) {
            return Err(DomainError::TodoAlreadyExists);
        }
        let next_id = hm.keys().max().map(|k| k + 1).unwrap_or(1);
        let now = Utc::now().naive_utc();
        let new_todo = Todo {
            id:next_id,
            body:todo.body().to_string(),
            completed: false,
            created_at: now,
            updated_at: now,
        };
        hm.insert(next_id, new_todo);
        Ok(())
    }

    #[tracing::instrument(name = "Updating todo in hashmap", skip_all)]
    async fn update(&self, id: i64, todo: UpdateTodo) -> Result<(), DomainError> {
        let mut hm = self.hm.write().await;
        let existing = hm.get_mut(&id).ok_or(DomainError::TodoNotFound)?;
        let now = Utc::now().naive_utc();
        existing.body = todo.body().to_string();
        existing.updated_at = now;
        Ok(())
    }

    #[tracing::instrument(name = "Deleting todo in hashmap", skip_all)]
    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let mut hm = self.hm.write().await;
        hm.remove(&id).ok_or(DomainError::TodoNotFound)?;
        Ok(())
    }
}

