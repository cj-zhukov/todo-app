use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use crate::db::{
    db::Todo, 
    db_write_ops::{CreateTodo, UpdateTodo}, 
    error::TodoStoreError
};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Option<Vec<Todo>>,
}

pub async fn todo_list(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let data = Todo::list(pool).await
        .map_err(|e| AppError::UnexpectedError(e.into()))?;

    let data = match data.is_empty() {
        true => None,
        false => Some(data)
    };
    
    let res = Json(Response {
        message: "Listing todos".to_string(),
        content: data,
    });

    Ok((StatusCode::OK, res))
}

pub async fn todo_read(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = match Todo::read_id(pool, id).await {
        Ok(todo) => todo,
        Err(e) => match e {
            TodoStoreError::TodoNotFound => return Err(AppError::TodoNotFound),
            TodoStoreError::UnexpectedError(e) => return Err(AppError::UnexpectedError(e.into())),
            TodoStoreError::TodoAlreadyExists => return Err(AppError::TodoAlreadyExists),
            TodoStoreError::InvalidCredentials => return Err(AppError::InvalidCredentials),
        }
    };

    let res = Json(Response {
        message: format!("Reading todo id: {}", id),
        content: Some(vec![data]),
    });

    Ok((StatusCode::OK, res))
}

pub async fn todo_create(
    State(pool): State<PgPool>, 
    Json(new_todo): Json<CreateTodo>,
) ->  Result<impl IntoResponse, AppError> {
    if let Ok(_todo) = Todo::read_body(pool.clone(), new_todo.body()).await {
        return Err(AppError::TodoAlreadyExists);
    }

    Todo::create(pool, new_todo).await
        .map_err(|e| AppError::UnexpectedError(e.into()))?;

    let res = Json(Response {
        message: format!("Todo created successfully"),
        content: None
    });

    Ok((StatusCode::CREATED, res))
}

pub async fn todo_update(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>, 
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, AppError> {
    match Todo::read_id(pool.clone(), id).await {
        Ok(_) => (),
        Err(e) => match e {
            TodoStoreError::TodoNotFound => return Err(AppError::TodoNotFound),
            TodoStoreError::UnexpectedError(e) => return Err(AppError::UnexpectedError(e.into())),
            TodoStoreError::TodoAlreadyExists => return Err(AppError::TodoAlreadyExists),
            TodoStoreError::InvalidCredentials => return Err(AppError::InvalidCredentials),
        }
    };

    Todo::update(pool, id, updated_todo).await
        .map_err(|e| AppError::UnexpectedError(e.into()))?;

    let res = Json(Response {
        message: format!("Todo id: {} updated successfully", id),
        content: None
    });

    Ok((StatusCode::OK, res))
}

pub async fn todo_delete(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    match Todo::read_id(pool.clone(), id).await {
        Ok(_) => (),
        Err(e) => match e {
            TodoStoreError::TodoNotFound => return Err(AppError::TodoNotFound),
            TodoStoreError::UnexpectedError(e) => return Err(AppError::UnexpectedError(e.into())),
            TodoStoreError::TodoAlreadyExists => return Err(AppError::TodoAlreadyExists),
            TodoStoreError::InvalidCredentials => return Err(AppError::InvalidCredentials),
        }
    };

    Todo::delete(pool, id).await
        .map_err(|e| AppError::UnexpectedError(e.into()))?;

    let res = Json(Response {
        message: format!("Todo id: {} deleted successfully", id),
        content: None
    });

    Ok((StatusCode::OK, res))
}