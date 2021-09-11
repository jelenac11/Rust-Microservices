use crate::connection::Pool;
use crate::model::{NewPost, PostDTO};
use crate::model::Post;
use crate::repository;
use actix_web::web::{Data, Json, Path};
use actix_web::web;
use actix_web::{error, Error, HttpResponse, HttpRequest};
use std::env;
use crate::utils;

#[derive(Deserialize)]
pub struct AverageRate {
    pub post_id: i32,
    pub average_rate: f32
}

#[get("/api/posts")]
pub async fn get_all_posts(pool: Data<Pool>) -> Result<HttpResponse, Error> {
    info!("All posts requested");
    let connection = pool.get().expect("Connection from pool");
    repository::get_all_posts(&connection)
        .map(|post| {
            info!("Successfully read all posts");
            HttpResponse::Ok().json(post)
        })
        .map_err(|error| {
            debug!("{}", format!("Error occured: {:?}", error));
            error_response(error)
        })
}

#[post("/api/posts")]
pub async fn create_post(post_dto: Json<PostDTO>, pool: Data<Pool>, http_request: HttpRequest) -> Result<HttpResponse, Error> {
    info!("Creating new post requested");
    let connection = pool.get().expect("Connection from pool");
    let uid = utils::get_user_id(http_request).unwrap();
    let post = post_dto.into_inner();
    let new_post = NewPost {
        title: post.title.clone(),
        text: post.text.clone(),
        rate: 0.0,
        author_id: uid
    };
    repository::create_post(new_post, &connection)
        .map(|post|{
            info!("New post successfully added");
            post_created(post)
        })
        .map_err(|error| {
            debug!("{}", format!("Error occured: {:?}", error));
            error_response(error)
        })
}

#[get("/api/posts/{id}")]
pub async fn get_post(id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    info!("Getting post by id requested");
    let connection = pool.get().expect("Connection from pool");
    repository::get_post(id.into_inner(), &connection)
        .map(|post| {
            info!("Successfully got post by id");
            post_ok(post)
        })
        .map_err(|error| {
            debug!("{}", format!("Error occured: {:?}", error));
            error_response(error)
        })
}

#[get("/api/posts/author")]
pub async fn get_posts_by_author_id(pool: Data<Pool>, http_request: HttpRequest) -> Result<HttpResponse, Error> {
    info!("Getting all posts by author requested");
    let connection = pool.get().expect("Connection from pool");
    let uid = utils::get_user_id(http_request).unwrap();
    repository::get_posts_by_author_id(uid, &connection)
        .map(|posts| {
            info!("Successfully got posts by author");
            HttpResponse::Ok().json(posts)
        })
        .map_err(|error|{
            debug!("{}", format!("Error occured: {:?}", error));
            error_response(error)
        })
}


#[delete("/api/posts/{id}")]
pub async fn delete_post(id: Path<i32>, pool: Data<Pool>, http_request: HttpRequest) -> Result<HttpResponse, Error> {
    info!("Deleting post requested");
    let connection = pool.get().expect("Connection from pool");
    let uid = utils::get_user_id(http_request).unwrap();
    let post = repository::get_post(id.0.clone(), &connection);
    if let Ok(existed_post) = post {
        info!("Post for deleting found");
        if existed_post.author_id == uid {
            return repository::delete_post(id.into_inner(), &connection)
                        .map(|_| {
                            info!("Post successfully deleted");
                            HttpResponse::NoContent().finish()
                        })
                        .map_err(|error| {
                            debug!("{}", format!("Error occured: {:?}", error));
                            error_response(error)
                        });
        } else {
            info!("User that is not author of post tried to delete it");
            return Err(error::ErrorBadRequest("Only author of post can delete post!".to_string()));
        }
    }
    info!("Post for deleting doesn't exist");
    Err(error::ErrorNotFound(format!("Post with id {} not found!", id)))
}

#[put("/api/posts")]
pub async fn update_average_rate(pool: Data<Pool>, web::Query(average_rate): web::Query<AverageRate>) -> Result<HttpResponse, Error> {
    info!("Updating average rate of post requested");
    let connection = pool.get().expect("Connection from pool");
    repository::update_average_rate(average_rate, &connection)
        .map(|post| {
            info!("Average rate of post successfully updated");
            HttpResponse::Ok().json(post.rate)
        })
        .map_err(|error| {
            debug!("{}", format!("Error occured: {:?}", error));
            error_response(error)
        })
}

fn post_created(post: Post) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/posts/{id}",
                host = host(),
                port = port(),
                id = post.id
            )
            .to_string(),
        )
        .json(post)
}

fn host() -> String {
    env::var("ADDRESS").expect("ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("PORT must be set")
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
