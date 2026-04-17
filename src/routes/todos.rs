use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{AppState, domain::{todo::Todo, todo_repository::{CreateTodo, UpdateTodo}}, utils::helpers::json_response};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Option<Vec<Todo>>,
}

#[tracing::instrument(name = "todo_list", skip(state))]
pub async fn todo_list(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let data = state.repo.list().await?;
    let data = (!data.is_empty()).then_some(data);
    Ok(json_response("Listing todos", data, StatusCode::OK))
}

#[tracing::instrument(name = "todo_read", skip(state))]
pub async fn todo_read(
    State(state): State<AppState>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = state.repo.read_id(id).await?;
    Ok(json_response(format!("Reading todo id: {}", id), Some(vec![data]), StatusCode::OK))
}

#[tracing::instrument(name = "todo_create", skip(state))]
pub async fn todo_create(
    State(state): State<AppState>, 
    Json(new_todo): Json<CreateTodo>,
) ->  Result<impl IntoResponse, AppError> {
    if let Ok(_todo) = state.repo.read_body(new_todo.body()).await {
        return Err(AppError::TodoAlreadyExists);
    }
    state.repo.create(new_todo).await?;
    Ok(json_response("Todo created successfully", None, StatusCode::CREATED))
}

#[tracing::instrument(name = "todo_update", skip(state))]
pub async fn todo_update(
    State(state): State<AppState>, 
    Path(id): Path<i64>, 
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let _ = state.repo.read_id(id).await?;
    state.repo.update(id, updated_todo).await?;
    Ok(json_response(format!("Todo id: {} updated successfully", id), None, StatusCode::OK))
}

#[tracing::instrument(name = "todo_delete", skip(state))]
pub async fn todo_delete(
    State(state): State<AppState>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let _ = state.repo.read_id(id).await?;
    state.repo.delete(id).await?;
    Ok(json_response(format!("Todo id: {} deleted successfully", id), None, StatusCode::OK))
}
