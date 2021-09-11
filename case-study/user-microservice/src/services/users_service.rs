use crate::{
    config::db::Pool,
    errors::error::AuthError,
    models::role::Role,
    models::token::Token,
    models::user::{LoginDTO, UserDTO},
    repository::user_repository,
    services::email_service,
    utils::utils,
};
use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn signup(user: UserDTO, pool: &web::Data<Pool>, role: String) -> Result<String, AuthError> {
    let user = UserDTO {
        password: hash(&user.password, DEFAULT_COST).unwrap(),
        role: role,
        ..user
    };
    let email = user.email.clone();
    let name = user.firstname.clone();
    let username = user.username.clone();
    match user_repository::signup(user, &pool.get().unwrap()) {
        Ok(_) => {
            info!(
                "{}",
                format!("User with username {} successfully registered", &username)
            );
            //email_service::send_mail(email, "Registration".to_string(), format!("Hello {}, you have successfully registered to Jelena's posts! Thank you", name))?;
            Ok(format!(
                "User with username {} successfully registered",
                username
            ))
        }
        Err(error) => {
            debug!("{}", format!("Unsuccessfull signup: {}", error.to_string()));
            Err(AuthError::from(error))
        }
    }
}

pub fn login(login: LoginDTO, pool: &web::Data<Pool>) -> Result<Token, AuthError> {
    if let Ok(user) = user_repository::find_user_by_email(&login.email, &pool.get().unwrap()) {
        if verify(login.password, &user.password).unwrap() {
            let role = Role::from_str(user.role.as_str());
            info!(
                "{}",
                format!("Successfull login with username {}", &login.email)
            );
            return Ok(Token {
                token: utils::create_token(user.id, user.username, role),
            });
        } else {
            debug!(
                "{}",
                format!(
                    "Unsuccessfull login, wrong password for username {}",
                    &login.email
                )
            );
            return Err(AuthError::GenericError(String::from(
                "Password not correct!Try again!",
            )));
        }
    }
    debug!(
        "{}",
        format!(
            "Unsuccessfull login, wrong credentials - username {}",
            &login.email
        )
    );
    Err(AuthError::NotFound(format!(
        "User with email {} does not exist!",
        &login.email
    )))
}
