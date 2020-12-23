use serde::Serialize;
use std::string::ToString;
use thiserror::Error;
use warp::http::StatusCode;
use warp::reply::Response;

use crate::server::http_response::HttpResponse;

/// A `Result` wrapper which returns a `AppError` variant
/// as the `Result::Err` variant
pub type Result<T> = std::result::Result<T, AppError>;

/// `MSendError` represents all possible application
/// errors.
///
/// `MSendError` trait is in charge of making sure an error
/// is capable to be a HTTP Error (`HttpResponse`) and also
/// an error variant for the specific application logic
pub trait MSendError: std::marker::Sized + std::fmt::Display + std::marker::Send {
    /// Retrieves the error message
    fn message(&self) -> String {
        self.to_string()
    }

    /// Creates a `HttpResponse` instance from `Self`
    fn into_http<T>(&self) -> HttpResponse<T>
    where
        T: std::marker::Sized + std::marker::Send + Serialize;

    /// Creates a`warp::reply::Response` from `Self`
    fn into_response(&self) -> Response {
        self.into_http::<String>().into()
    }
}

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Database transaction did not succeed: {0}")]
    DatabaseError(String),
    #[error("Unexpected server error: {0}")]
    UnexpectedServerError(String),
    #[error("Invalid credentials were provided")]
    InvalidCredentials,
    #[error("Username `{0}` is taken")]
    UsernameTaken(String),
    #[error("Invalid basic authentication header: {0}")]
    InvalidBasicAuthHeader(String),
    #[error("Unsupported Content-Type for \"image\": {0}")]
    UnsupportedImage(String),
    #[error("An error ocurred reading the image file: {0}")]
    ReadImageError(String),
    #[error("Failed to read input (client) message: {0}")]
    ReadMessageError(String),
    #[error("Failed to write output (server) message: {0}")]
    WriteMessageError(String),
}

impl MSendError for AppError {
    fn into_http<T>(&self) -> HttpResponse<T>
    where
        T: Sized + Send + serde::Serialize,
    {
        match self {
            AppError::DatabaseError(msg) => {
                error!("{}", msg);
                HttpResponse::new(&msg, StatusCode::INTERNAL_SERVER_ERROR)
            }
            AppError::UnexpectedServerError(msg) => {
                error!("{}", msg);
                HttpResponse::new(&msg, StatusCode::INTERNAL_SERVER_ERROR)
            }
            AppError::InvalidCredentials => {
                error!("{}", &self.message());
                HttpResponse::new(&self.message(), StatusCode::FORBIDDEN)
            }
            AppError::UsernameTaken(_) => {
                error!("{}", &self.message());
                HttpResponse::new(&self.message(), StatusCode::BAD_REQUEST)
            }
            AppError::InvalidBasicAuthHeader(msg) => {
                error!("{}", msg);
                HttpResponse::new(&msg, StatusCode::BAD_REQUEST)
            }
            AppError::UnsupportedImage(_) => {
                error!("{}", &self.message());
                HttpResponse::new(&self.message(), StatusCode::BAD_REQUEST)
            }
            AppError::ReadImageError(msg) => {
                error!("{}", msg);
                HttpResponse::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
            }
            AppError::ReadMessageError(msg) | AppError::WriteMessageError(msg) => {
                error!("{}", msg);
                HttpResponse::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl From<sqlx::error::Error> for AppError {
    fn from(e: sqlx::error::Error) -> Self {
        match e.as_database_error() {
            Some(db_error) => AppError::DatabaseError(db_error.to_string()),
            None => {
                error!("{:?}", e);
                AppError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}
