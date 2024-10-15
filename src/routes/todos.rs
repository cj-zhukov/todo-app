use crate::db::{db::Todo, db_write_ops::{CreateTodo, UpdateTodo}};

use anyhow::Result;
use axum::{extract::{Path, State}, response::IntoResponse, Json};
use sqlx::PgPool;

pub async fn todo_list(State(pool): State<PgPool>) -> Result<impl IntoResponse, Json<Vec<Todo>>> {
    let res = Todo::list(pool).await.map(Json::from).unwrap();

    Ok(res)
}

pub async fn todo_read(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<impl IntoResponse, Json<Todo>> {
    let res = Todo::read(pool, id).await.map(Json::from).unwrap();

    Ok(res)
}

pub async fn todo_create(State(pool): State<PgPool>, Json(new_todo): Json<CreateTodo>) -> Result<impl IntoResponse, Json<Todo>> {
    let res = Todo::create(pool, new_todo).await.map(Json::from).unwrap();

    Ok(res)
}

pub async fn todo_update(State(pool): State<PgPool>, Path(id): Path<i64>, Json(updated_todo): Json<UpdateTodo>) -> Result<impl IntoResponse, Json<Todo>> {
    let res = Todo::update(pool, id, updated_todo).await.map(Json::from).unwrap();

    Ok(res)
}

pub async fn todo_delete(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<impl IntoResponse, Json<Todo>> {
    let res = Todo::delete(pool, id).await.map(Json::from).unwrap();

    Ok(res)
}