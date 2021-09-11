use crate::{
    models::rate::RateDTO,
    constants,
    utils::client_response,
};
use actix_web::{web, HttpResponse, web::Path, HttpRequest};
use awc;

// POST api/rates
pub async fn rate_post(rate_dto: web::Json<RateDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::REVIEW_MS, "/api/rates");
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
        .header("Authorization", token.unwrap())
        .send_json(&rate_dto.into_inner())
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

// GET api/rates/post/{id}
pub async fn get_user_rate_for_post(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", &constants::REVIEW_MS, "/api/rates/post/", id.into_inner());
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
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
