use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{db::{
    db_write_ops::{CreateTodo, UpdateTodo}, todo::Todo 
}, utils::helpers::json_response};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Option<Vec<Todo>>,
}

#[tracing::instrument(name = "todo_list")]
pub async fn todo_list(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let data = Todo::list(pool).await?;
    let data = match data.is_empty() {
        true => None,
        false => Some(data)
    };
    Ok(json_response("Listing todos", data, StatusCode::OK))
}

#[tracing::instrument(name = "todo_read")]
pub async fn todo_read(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = Todo::read_id(pool, id).await?;
    Ok(json_response(format!("Reading todo id: {}", id), Some(vec![data]), StatusCode::OK))
}

#[tracing::instrument(name = "todo_create")]
pub async fn todo_create(
    State(pool): State<PgPool>, 
    Json(new_todo): Json<CreateTodo>,
) ->  Result<impl IntoResponse, AppError> {
    if let Ok(_todo) = Todo::read_body(pool.clone(), new_todo.body()).await {
        return Err(AppError::TodoAlreadyExists);
    }
    Todo::create(pool, new_todo).await?;
    Ok(json_response("Todo created successfully", None, StatusCode::CREATED))
}

#[tracing::instrument(name = "todo_update")]
pub async fn todo_update(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>, 
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let _ = Todo::read_id(pool.clone(), id).await?;
    Todo::update(pool, id, updated_todo).await?;
    Ok(json_response(format!("Todo id: {} updated successfully", id), None, StatusCode::OK))
}

#[tracing::instrument(name = "todo_delete")]
pub async fn todo_delete(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let _ = Todo::read_id(pool.clone(), id).await?;
    Todo::delete(pool, id).await?;
    Ok(json_response(format!("Todo id: {} deleted successfully", id), None, StatusCode::OK))
}
