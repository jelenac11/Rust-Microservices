use crate::actors::count::Count;
use crate::actors::db::{Create, Delete, Get, GetPosts, Publish, Update};
use crate::model::model::AppState;
use crate::model::model::NewPost;
use crate::model::model::Post;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, Responder};
use std::env;

#[get("/posts")]
async fn all_posts(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    count_up(&state, "get /posts").await;
    match db.send(GetPosts).await {
        Ok(Ok(posts)) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[get("/posts/{id}")]
async fn get_post(Path(id): Path<i32>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    count_up(&state, "get /posts/id").await;
    match db.send(Get { id }).await {
        Ok(Ok(post)) => HttpResponse::Ok().json(post),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[post("/posts")]
async fn create_post(post: Json<NewPost>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    count_up(&state, "post /posts").await;
    match db
        .send(Create {
            title: post.title,
            description: post.description,
        })
        .await
    {
        Ok(Ok(post)) => post_created(post),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[post("/posts/{id}/publish")]
async fn publish_post(Path(id): Path<i32>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    count_up(&state, "post /posts/{id}/publish").await;
    match db.send(Publish { id }).await {
        Ok(Ok(post)) => HttpResponse::Ok().json(post),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[delete("/posts/{id}")]
async fn delete_post(Path(id): Path<i32>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    count_up(&state, "delete /posts/{id}").await;
    match db.send(Delete { id }).await {
        Ok(Ok(_)) => HttpResponse::Ok().finish(),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
}

#[put("/posts/{id}")]
async fn update_post(
    Path(id): Path<i32>,
    post: Json<NewPost>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    count_up(&state, "put /posts/{id}").await;
    match db
        .send(Update {
            id,
            title: post.title,
            description: post.description,
        })
        .await
    {
        Ok(Ok(_)) => HttpResponse::NoContent().finish(),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong"),
    }
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

async fn count_up(state: &AppState, path: &str) -> () {
    let path = path.to_string();
    let count = state.count.clone();
    match count.send(Count(path.clone())).await {
        Ok(value) => {
            println!("total requests for '{}' is {}", path, value);
        }
        Err(error) => {
            println!("{:?}", error)
        }
    }
}
