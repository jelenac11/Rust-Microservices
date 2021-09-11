use actix_web::web;
use crate::endpoints::*;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service (
        web::scope("/api")
            .service(healthcheck::healthcheck)
            .service(
                web::scope("/comments")
                    .service(
                        web::resource("")
                            .route(web::post().to(comments_controller::create_comment))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(comments_controller::get_comments_for_post))
                            .route(web::delete().to(comments_controller::delete_comment))
                    )
            )
            .service(
                web::scope("/rates")
                    .service(
                        web::resource("")
                            .route(web::post().to(rates_controller::rate_post))
                    )
                    .service(
                        web::resource("/post/{id}")
                            .route(web::get().to(rates_controller::get_user_rate_for_post))
                    )
            )
    );
}