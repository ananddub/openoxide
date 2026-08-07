use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    #[error("Invalid value for field {field}: {message}")]
    InvalidField {
        field: &'static str,
        message: String,
    },

    #[error("Unsupported source type: {0}")]
    UnsupportedSourceType(String),

    #[error("Database error occurred: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Git adapter error: {0}")]
    GitAdapterError(#[from] os::git::provider::AdapterError),
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("Adapter error: {0}")]
    Adapter(#[from] AdapterError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Execution failed: {0}")]
    Execution(String),

    #[error("Invalid state transition")]
    InvalidState,
}
