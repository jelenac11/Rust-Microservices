use crate::model::comment::Comment;
use actix_web::{error, Error, HttpResponse};
use std::env;

pub fn error_response(error: diesel::result::Error) -> Error {
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}

pub fn comment_created(comment: Comment) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/comments/{id}",
                host = host(),
                port = port(),
                id = comment.id
            )
            .to_string(),
        )
        .json(comment)
}

fn host() -> String {
    env::var("ADDRESS").expect("ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("PORT must be set")
}
