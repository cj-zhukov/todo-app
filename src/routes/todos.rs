use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

use crate::db::{
    db::Todo, 
    db_write_ops::{CreateTodo, UpdateTodo}
};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub content: Option<Vec<Todo>>,
}

pub async fn todo_list(State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let data = Todo::list(pool).await
        .map_err(|_| AppError::UnexpectedError)?;
    
    let res = Json(Response {
        message: "Listing todos".to_string(),
        content: Some(data),
    });

    Ok((StatusCode::OK, res))
}

pub async fn todo_read(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = match Todo::read(pool, id).await {
        Ok(res) => res,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::TodoNotFound),
            _ => return Err(AppError::UnexpectedError),
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
    let data = Todo::create(pool, new_todo).await
        .map_err(|_| AppError::UnexpectedError)?;

    let res = Json(Response {
        message: format!("Todo id: {} created successfully", data.id),
        content: Some(vec![data])
    });

    Ok((StatusCode::CREATED, res))
}

pub async fn todo_update(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>, 
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let data = match Todo::update(pool, id, updated_todo).await {
        Ok(res) => res,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::TodoNotFound),
            _ => return Err(AppError::UnexpectedError),
        }
    };

    let res = Json(Response {
        message: format!("Todo id: {} updated successfully", id),
        content: Some(vec![data])
    });

    Ok((StatusCode::OK, res))
}

pub async fn todo_delete(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let data = match Todo::delete(pool, id).await {
        Ok(res) => res,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Err(AppError::TodoNotFound),
            _ => return Err(AppError::UnexpectedError),
        }
    };

    let res = Json(Response {
        message: format!("Todo id: {} deleted successfully", id),
        content: Some(vec![data])
    });

    Ok((StatusCode::OK, res))
}