use crate::connection::Pool;
use crate::model::NewPost;
use crate::model::Post;
use crate::repository;
use actix_web::web::{Data, Json, Path};
use actix_web::{error, Error, HttpResponse};
use std::env;

#[get("/posts")]
pub async fn all_posts(pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::get_all_posts(&connection)
        .map(|post| HttpResponse::Ok().json(post))
        .map_err(|error| error_response(error))
}

#[post("/posts")]
pub async fn create_post(new_post: Json<NewPost>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::create_post(new_post.into_inner(), &connection)
        .map(|post| post_created(post))
        .map_err(|error| error_response(error))
}

#[get("/posts/{id}")]
pub async fn get_post(id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::get_post(id.into_inner(), &connection)
        .map(|post| post_ok(post))
        .map_err(|error| error_response(error))
}

#[put("/posts/{id}")]
pub async fn update_post(
    id: Path<i32>,
    post: Json<Post>,
    pool: Data<Pool>,
) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::update_post(id.into_inner(), post.into_inner(), &connection)
        .map(|post| post_ok(post))
        .map_err(|error| error_response(error))
}

#[post("/posts/{id}")]
pub async fn publish_post(id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::publish_post(id.into_inner(), &connection)
        .map(|post| post_ok(post))
        .map_err(|error| error_response(error))
}

#[delete("/posts/{id}")]
pub async fn delete_post(id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Connection from pool");
    repository::delete_post(id.into_inner(), &connection)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|error| error_response(error))
}

fn post_created(post: Post) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/posts/{id}",
                host = host(),
                port = port(),
                id = post.id
            )
            .to_string(),
        )
        .json(post)
}

fn host() -> String {
    env::var("ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("ROCKET_PORT must be set")
}

fn error_response(error: diesel::result::Error) -> Error {
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}

fn post_ok(post: Post) -> HttpResponse {
    HttpResponse::Ok().json(post)
}
