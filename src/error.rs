use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use color_eyre::eyre::Report;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Domain error")]
    Domain(#[from] DomainError),

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

        let (status, message) = match self {
            AppError::Domain(e) => match e {
                DomainError::TodoAlreadyExists => (StatusCode::CONFLICT, e.to_string()),
                DomainError::TodoNotFound => (StatusCode::NOT_FOUND, e.to_string()),
                DomainError::InvalidCredentials => (StatusCode::BAD_REQUEST, e.to_string()),
                DomainError::UnexpectedError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                }
            },
            AppError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: message.to_string(),
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
