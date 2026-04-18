use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Todo already exists")]
    TodoAlreadyExists,

    #[error("Todo not found")]
    TodoNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}
