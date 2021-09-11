use actix_web::HttpRequest;
use chrono::{Duration, Local};
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use jsonwebtoken::{encode, EncodingKey, Header};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::{
    models::role::Role,
};

lazy_static! {
    static ref JWT_SECRET_KEY: String = std::env::var("JWT_SECRET_KEY").unwrap_or_else(|_| "jwt_secret_key".to_string());
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub uid: i32,
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

pub fn create_token(uid: i32, username: String, role: Role) -> String {
    info!("{}", format!("Creating token for user with username {}", &username));
    let exp_time = Local::now() + Duration::minutes(3600);

    let claims = Claims {
        uid: uid,
        sub: username,
        exp: exp_time.timestamp(),
        role: role.to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
    )
    .expect("Can't create token")
}

pub fn get_role(http_request: HttpRequest) -> Option<Role> {
    info!("Getting role from hhtp request header Authorization");
    http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();
                let token_data = decode_token(&jwt);
                Role::from_str(&token_data.claims.role)
            })
        })
}

fn decode_token(token: &str) -> TokenData<Claims> {
    info!("Decoding token...");
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}