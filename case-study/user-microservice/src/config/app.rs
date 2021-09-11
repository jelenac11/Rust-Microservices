use actix_web::web;
use crate::endpoints::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service (
        web::scope("/api")
            .service(healthcheck::healthcheck)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup")
                            .route(web::post().to(users_controller::signup))
                    )
                    .service(
                        web::resource("/login")
                            .route(web::post().to(users_controller::login))
                    )
                    .service(
                        web::resource("/role")
                            .route(web::post().to(users_controller::get_role))
                    )
                    .service(
                        web::resource("/admin")
                            .route(web::post().to(users_controller::add_admin))
                    )
        )
    );
}