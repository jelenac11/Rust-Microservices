use crate::{
    config::db::Pool,
    errors::error::AuthError,
    models::{
        role::Role,
        user::{LoginDTO, UserDTO},
    },
    services::users_service,
    utils,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use validator::Validate;

// POST api/auth/signup
pub async fn signup(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    trace!("Signup requested");
    let user = user_dto.into_inner();
    user.validate()
        .map_err(|e| return AuthError::GenericError(e.to_string()))?;
    match users_service::signup(user, &pool, Role::User.to_string()) {
        Ok(message) => Ok(HttpResponse::Created().json(message)),
        Err(err) => Err(err),
    }
}

// POST api/auth/admin
pub async fn add_admin(
    user_dto: web::Json<UserDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    trace!("New admin registration requested");
    let user = user_dto.into_inner();
    user.validate()
        .map_err(|e| return AuthError::GenericError(e.to_string()))?;
    match users_service::signup(user, &pool, Role::Admin.to_string()) {
        Ok(message) => Ok(HttpResponse::Ok().json(message)),
        Err(err) => Err(err),
    }
}

// POST api/auth/login
pub async fn login(
    login_dto: web::Json<LoginDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, AuthError> {
    trace!("Login requested");
    match users_service::login(login_dto.into_inner(), &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(token_res)),
        Err(err) => Err(err),
    }
}

//proba za dekod tokena
pub async fn get_role(_pool: web::Data<Pool>, http_request: HttpRequest) -> Result<HttpResponse> {
    let role = utils::utils::get_role(http_request);
    Ok(HttpResponse::Ok().json(role))
}

#[cfg(test)]
mod tests {
    use crate::{config, App};
    use actix_cors::Cors;
    use actix_web::{http, http::StatusCode, test};
    use http::header;

    #[actix_rt::test]
    async fn test_signup_ok() {
        let pool = config::db::migrate_and_config_db();
        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser1","email":"user1@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"123456", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
        let data = test::read_body(resp).await;
        assert_eq!(
            data,
            "\"User with username useruser1 successfully registered\""
        );
    }

    #[actix_rt::test]
    async fn test_signup_duplicate_user() {
        let pool = config::db::migrate_and_config_db();
        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser2","email":"user2@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"123456", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
        let data = test::read_body(resp).await;
        assert_eq!(
            data,
            "\"User with username useruser2 successfully registered\""
        );

        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser2","email":"user2@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"123456", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let data = test::read_body(resp).await;
        assert_eq!(data, "\"Key (username)=(useruser2) already exists.\"");
    }

    #[actix_rt::test]
    async fn test_signup_password_less_than_six_characters_long() {
        let pool = config::db::migrate_and_config_db();

        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .send_wildcard()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser3","email":"user3@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"12345", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_login_ok() {
        let pool = config::db::migrate_and_config_db();

        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser4","email":"user4@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"123456", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/login")
            .set(header::ContentType::json())
            .set_payload(r#"{"email":"user4@gmail.com","password":"123456"}"#.as_bytes())
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_login_with_wrong_password() {
        let pool = config::db::migrate_and_config_db();

        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        test::TestRequest::post()
            .uri("/api/auth/signup")
            .set(header::ContentType::json())
            .set_payload(r#"{"username":"useruser5","email":"user5@gmail.com", "firstname": "Adminko", "lastname": "Admirovic", "password":"123456", "role":"ROLE_USER"}"#.as_bytes())
            .send_request(&mut app)
            .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/login")
            .set(header::ContentType::json())
            .set_payload(r#"{"email":"user5@gmail.com","password":"123457"}"#.as_bytes())
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let data = test::read_body(resp).await;
        assert_eq!(data, "\"Password not correct!Try again!\"");
    }

    #[actix_rt::test]
    async fn test_login_non_existing_user() {
        let pool = config::db::migrate_and_config_db();

        let mut app = test::init_service(
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                        .allowed_header(http::header::CONTENT_TYPE)
                        .max_age(3600),
                )
                .data(pool.clone())
                .configure(crate::config::app::config_services),
        )
        .await;

        let resp = test::TestRequest::post()
            .uri("/api/auth/login")
            .set(header::ContentType::json())
            .set_payload(r#"{"email":"user6@gmail.com","password":"123456"}"#.as_bytes())
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let data = test::read_body(resp).await;
        assert_eq!(data, "\"User with email user6@gmail.com does not exist!\"");
    }
}
