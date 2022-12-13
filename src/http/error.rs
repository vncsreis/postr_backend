use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request path not found")]
    NotFound,
    #[error("An error ocurred with the database")]
    SqlxError(#[from] sqlx::Error),
    #[error("Server error")]
    AnyhowError(#[from] anyhow::Error),
    #[error("Authentication required")]
    Unauthorized,
    #[error("Token Error")]
    JWTError(#[from] jsonwebtoken::errors::Error),
    #[error("Invalid token")]
    InvalidToken,
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized | Self::InvalidToken => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let message = match self {
            Self::SqlxError(ref e) => format!("SQLx error: {:?}", e),
            Self::AnyhowError(ref e) => format!("Server error: {:?}", e),
            Self::JWTError(ref e) => format!("Token error: {:?}", e),
            Self::Unauthorized | Self::InvalidToken => format!("Invalid or missing token"),
            _ => format!("Internal server error"),
        };

        (self.status_code(), message).into_response()
    }
}
