use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{db::todo::Todo, routes::todos::Response};

pub fn json_response(
    message: impl Into<String>, 
    content: Option<Vec<Todo>>, 
    status: StatusCode,
) -> impl IntoResponse {
    (
        status,
        Json(Response {
            message: message.into(),
            content,
        }),
    )
}
