use crate::{
    constants,
    models::user::{LoginDTO, UserDTO},
    utils::client_response,
};
use actix_web::{web, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use awc;

// POST api/auth/signup
pub async fn signup(user_dto: web::Json<UserDTO>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::USER_MS, "/api/auth/signup");
    let resp = client.post(url).send_json(&user_dto.into_inner()).await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// POST api/auth/admin
#[has_any_role("ADMIN")]
pub async fn add_admin(user_dto: web::Json<UserDTO>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::USER_MS, "/api/auth/admin");
    let resp = client.post(url).send_json(&user_dto.into_inner()).await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// POST api/auth/login
pub async fn login(login_dto: web::Json<LoginDTO>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &constants::USER_MS, "/api/auth/login");
    let resp = client.post(url).send_json(&login_dto.into_inner()).await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}
