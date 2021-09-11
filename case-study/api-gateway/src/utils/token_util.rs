use actix_web::dev::{RequestHead, ServiceRequest};
use actix_web::Error;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref JWT_SECRET_KEY: String =
        std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");
}

#[derive(Deserialize)]
pub struct Claims {
    pub uid: i32,
    pub sub: String,
    pub exp: i64,
    pub role: String,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let token_data = decode_token(credentials.token());
    let mut roles = Vec::new();
    roles.push(token_data.claims.role);
    req.attach(roles);
    Ok(req)
}

pub fn check_role(head: &RequestHead, role: &str) -> bool {
    let token = head
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .ok()
        .unwrap();
    let jwt_start_index = "Bearer ".len();
    let jwt = token[jwt_start_index..token.len()].to_string();
    let token_data = decode_token(&jwt);
    token_data.claims.role == role
}

fn decode_token(token: &str) -> TokenData<Claims> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
        &Validation::default(),
    )
    .expect("Can't decode token")
}
