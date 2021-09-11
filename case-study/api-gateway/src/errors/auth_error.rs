use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    #[display(fmt = "AuthenticationError: {}", _0)]
    AuthenticationError(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),

    #[display(fmt = "UniqueViolation: {}", _0)]
    UniqueViolation(String),

    #[display(fmt = "ProcessError: {}", _0)]
    ProcessError(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),

            AuthError::AuthenticationError(ref message) => {
                HttpResponse::Unauthorized().json(message)
            }

            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::UniqueViolation(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::ProcessError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
        }
    }
}
