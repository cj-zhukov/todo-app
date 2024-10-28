use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AppError {
    TodoAlreadyExists,
    TodoNotFound,
    InvalidCredentials,
    UnexpectedError,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::TodoAlreadyExists => (StatusCode::CONFLICT, "Todo already exists"),
            Self::TodoNotFound => (StatusCode::NOT_FOUND, "Todo not found"),
            Self::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            Self::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            Self::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            Self::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error"),
        };
        
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        
        (status, body).into_response()
    }
}