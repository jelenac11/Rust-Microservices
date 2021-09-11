use crate::{Context, Post, Response};
use hyper::StatusCode;
use std::sync::{Arc, RwLock};

pub async fn get_post_by_id(ctx: Context, posts_db: Arc<RwLock<Vec<Post>>>) -> Response {
    let param = match ctx.params.find("id") {
        Some(v) => v,
        None => "empty",
    };
    if param != "empty" {
        let posts = posts_db.read().unwrap();
        let idx = param.parse::<i32>().unwrap() as usize;
        if let Some(_) = posts.get(idx) {
            return Response::new(posts[idx].to_string().into());
        } else {
            return create_response(StatusCode::NOT_FOUND, "Post not found".to_string());
        }
    }
    create_response(
        StatusCode::BAD_REQUEST,
        "Param id is not specified".to_string(),
    )
}

pub async fn create_post(mut ctx: Context, posts_db: Arc<RwLock<Vec<Post>>>) -> Response {
    let body: crate::Post = match ctx.body_json().await {
        Ok(v) => v,
        Err(_) => {
            return create_response(
                StatusCode::BAD_REQUEST,
                "Body can not deserialize to Post struct".to_string(),
            );
        }
    };
    let mut posts = posts_db.write().unwrap();
    posts.push(body);
    create_response(StatusCode::CREATED, (posts.len() - 1).to_string())
}

pub async fn delete_post(ctx: Context, posts_db: Arc<RwLock<Vec<Post>>>) -> Response {
    let param = match ctx.params.find("id") {
        Some(v) => v,
        None => "empty",
    };
    if param != "empty" {
        let mut posts = posts_db.write().unwrap();
        let idx = param.parse::<i32>().unwrap() as usize;
        if let Some(_) = posts.get(idx) {
            posts.remove(idx);
            return create_response(StatusCode::OK, "Successfully deleted".to_string());
        } else {
            return create_response(StatusCode::NOT_FOUND, "Post not found".to_string());
        }
    }
    create_response(
        StatusCode::BAD_REQUEST,
        "Param id is not specified".to_string(),
    )
}

pub async fn update_post(mut ctx: Context, posts_db: Arc<RwLock<Vec<Post>>>) -> Response {
    let body: Post = match ctx.body_json().await {
        Ok(v) => v,
        Err(_) => {
            return create_response(
                StatusCode::BAD_REQUEST,
                "Body can not deserialize to Post struct".to_string(),
            );
        }
    };
    let param = match ctx.params.find("id") {
        Some(v) => v,
        None => "empty",
    };
    if param != "empty" {
        let mut posts = posts_db.write().unwrap();
        let idx = param.parse::<i32>().unwrap() as usize;
        if let Some(_) = posts.get(idx) {
            posts[idx] = body;
            return create_response(StatusCode::OK, "Successfully updated".to_string());
        } else {
            return create_response(StatusCode::NOT_FOUND, "Post not found".to_string());
        }
    }
    create_response(
        StatusCode::BAD_REQUEST,
        "Param id is not specified".to_string(),
    )
}

pub fn create_response(status: StatusCode, text: String) -> Response {
    return hyper::Response::builder()
        .status(status)
        .body(text.into())
        .unwrap();
}
