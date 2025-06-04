use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use color_eyre::eyre::Report;
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::db::error::TodoStoreError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Todo already exists")]
    TodoAlreadyExists,

    #[error("Todo not found")]
    TodoNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Incorrect credentials")]
    IncorrectCredentials,
    
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        
        let (status, error_message) = match self {
            Self::TodoAlreadyExists => (StatusCode::CONFLICT, "Todo already exists"),
            Self::TodoNotFound => (StatusCode::NOT_FOUND, "Todo not found"),
            Self::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            Self::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            Self::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };
        
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn std::error::Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

impl From<TodoStoreError> for AppError {
    fn from(err: TodoStoreError) -> Self {
        match err {
            TodoStoreError::TodoNotFound => AppError::TodoNotFound,
            TodoStoreError::TodoAlreadyExists => AppError::TodoAlreadyExists,
            TodoStoreError::InvalidCredentials => AppError::InvalidCredentials,
            TodoStoreError::UnexpectedError(e) => AppError::UnexpectedError(e),
        }
    }
}
