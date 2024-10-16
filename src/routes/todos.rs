use crate::{db::{db::Todo, db_write_ops::{CreateTodo, UpdateTodo}}, Response};

use anyhow::Result;
use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

pub async fn todo_list(State(pool): State<PgPool>) -> Result<impl IntoResponse, Json<Response>> {
    let data = Todo::list(pool).await.map_err(|_|
        Json(Response {
            message: "Failed listing todo".to_string(),
            content: None,
        })
    )?;
    
    let res = Json(Response {
        message: "Listing todo".to_string(),
        content: Some(data),
    });

    Ok(res)
}

pub async fn todo_read(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Json<Response>> {
    let data = Todo::read(pool, id).await.map_err(|_|
        Json(Response {
            message: format!("Failed listing todo id: {}", id),
            content: None,
        })
    )?;

    let res = Json(Response {
        message: "Listing todo".to_string(),
        content: Some(vec![data]),
    });

    Ok(res)
}

pub async fn todo_create(
    State(pool): State<PgPool>, 
    Json(new_todo): Json<CreateTodo>,
) -> Result<impl IntoResponse, Json<Response>> {
    let data = Todo::create(pool, new_todo).await.map_err(|_|
        Json(Response {
            message: "Failed creating todo".to_string(),
            content: None,
        })
    )?;

    let res = Json(Response {
        message: "Todo created successfully".to_string(),
        content: Some(vec![data])
    });

    Ok((StatusCode::CREATED, res))
}

pub async fn todo_update(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>, 
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<impl IntoResponse, Json<Response>> {
    let data = Todo::update(pool, id, updated_todo).await.map_err(|_|
        Json(Response {
            message: format!("Failed updating todo id: {}", id),
            content: None,
        })
    )?;

    let res = Json(Response {
        message: "Todo updated successfully".to_string(),
        content: Some(vec![data])
    });

    Ok(res)
}

pub async fn todo_delete(
    State(pool): State<PgPool>, 
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Json<Response>> {
    let data = Todo::delete(pool, id).await.map_err(|_|
        Json(Response {
            message: format!("Failed deleting todo id: {}", id),
            content: None,
        })
    )?;

    let res = Json(Response {
        message: "Todo deleted successfully".to_string(),
        content: Some(vec![data])
    });

    Ok(res)
}