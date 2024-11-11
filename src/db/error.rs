use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoStoreError {
    #[error("Todo already exists")]
    TodoAlreadyExists,

    #[error("Todo not found")]
    TodoNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for TodoStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::TodoAlreadyExists, Self::TodoAlreadyExists)
                | (Self::TodoNotFound, Self::TodoNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}