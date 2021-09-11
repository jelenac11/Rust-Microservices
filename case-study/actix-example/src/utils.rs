use actix_web::HttpRequest;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref JWT_SECRET_KEY: String = std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub uid: i32,
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

pub fn get_user_id(http_request: HttpRequest) -> Option<i32> {
    info!("Getting user id from http request");
    http_request
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();
                let token_data = decode_token(&jwt);
                token_data.claims.uid
            })
        })
}

fn decode_token(token: &str) -> TokenData<Claims> {
    info!("Decoding token");
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}

