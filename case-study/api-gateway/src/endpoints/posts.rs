use crate::{
    models::post::PostDTO,
    constants,
    utils::client_response,
};
use actix_web::{web, HttpResponse, web::Path, HttpRequest};
use awc;
use actix_web_grants::proc_macro::{has_any_role};

#[derive(Deserialize)]
pub struct Search {
    search: Option<String>,
}

// GET api/posts
pub async fn get_all(web::Query(key_word): web::Query<Search>) -> HttpResponse {
    let mut url = format!("{}{}", &constants::POST_MS, "/api/posts");
    match key_word.search {
        Some(word) => { url = format!("{}{}{}", &constants::SEARCH_MS, "/api/posts?search=", word); }
        None => {}
    }
    let client = awc::Client::new();
    let resp = client.get(url)
        .send()
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

// POST api/posts
pub async fn create_post(post_dto: web::Json<PostDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::POST_MS, "/api/posts");
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
        .header("Authorization", token.unwrap())
        .send_json(&post_dto.into_inner())
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

// GET api/posts/author
#[has_any_role("ADMIN")]
pub async fn get_by_author(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let url = format!("{}{}", &constants::POST_MS, "/api/posts/author");
    let resp = client.get(url)
        .header("Authorization", token.unwrap())
        .send()
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

// GET api/posts/{id}
pub async fn get_by_id(id: Path<i32>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", &constants::POST_MS, "/api/posts/", id.into_inner());
    let resp = client.get(url)
        .send()
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

// DELETE api/posts/{id}
#[has_any_role("ADMIN")]
pub async fn delete(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", &constants::POST_MS, "/api/posts/", id.into_inner());
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.delete(url)
        .header("Authorization", token.unwrap())
        .send()
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