use crate::{constants, models::comment::CommentDTO, utils::client_response};
use actix_web::{web, web::Path, HttpRequest, HttpResponse};
use awc;

// POST api/comments
pub async fn create_comment(comment_dto: web::Json<CommentDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::REVIEW_MS, "/api/comments");
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client
        .post(url)
        .header("Authorization", token.unwrap())
        .send_json(&comment_dto.into_inner())
        .await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// GET api/comments/{id}
pub async fn get_comments_for_post(id: Path<i32>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!(
        "{}{}{}",
        &constants::REVIEW_MS,
        "/api/comments/",
        id.into_inner()
    );
    let resp = client.get(url).send().await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// DELETE api/comments/{id}
pub async fn delete_comment(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!(
        "{}{}{}",
        &constants::REVIEW_MS,
        "/api/comments/",
        id.into_inner()
    );
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client
        .delete(url)
        .header("Authorization", token.unwrap())
        .send()
        .await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            println!("{}", error.to_string());
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}
