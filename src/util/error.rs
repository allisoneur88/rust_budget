use std::io;

use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {entity} with id {id:?}")]
    NotFound { entity: &'static str, id: Uuid },

    #[error("User not logged in")]
    NotLoggedIn,

    #[error("No budget selected")]
    NoBudgetSelected,

    #[error("No account selected")]
    NoAccountSelected,

    #[error("User already exists: {0}")]
    UserExists(String),

    #[error("Access forbidden")]
    Forbidden,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Config(#[from] toml::de::Error),
}

pub type AppResult<T> = Result<T, AppError>;
