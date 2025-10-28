use std::io;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Not found: {entity} with id {id}")]
    NotFound {
        entity: &'static str,
        id: uuid::Uuid,
    },

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("User with this name already exists:{0}")]
    UserExists(String),

    #[error("Unknown error: {0}")]
    Other(String),
}

pub type AppResult<T> = Result<T, AppError>;
