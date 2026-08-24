use axum::{http::StatusCode, response::IntoResponse};

use crate::response::ApiResponseBuilder;
use crate::response::EmptyResponseBody;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),

    #[error("Error parsing env due to {0}")]
    EnvError(String),

    #[error("{0}")]
    OperationFailed(String),

    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("GraphQL error: {0}")]
    GraphQLError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error(transparent)]
    LunarError(#[from] lunar::error::LunarError),

    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),

    #[error("invalid env was passed {0}")]
    InvalidEnv(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,

            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,

            AppError::GraphQLError(_) => StatusCode::BAD_REQUEST,
            AppError::OperationFailed(_) => StatusCode::BAD_REQUEST,
            AppError::EnvError(_)
            | AppError::InvalidEnv(_)
            | AppError::StartupError(_)
            | AppError::InternalError(_)
            | AppError::LunarError(_)
            | AppError::DatabaseError(_)
            | AppError::FileSystemError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<EmptyResponseBody>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
