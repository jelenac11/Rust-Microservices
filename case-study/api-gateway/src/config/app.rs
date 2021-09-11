use crate::endpoints::*;
use crate::token_util;
use actix_web::{guard, web};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(token_util::validator);
    cfg.service(
        web::scope("/api")
            .service(healthcheck::healthcheck)
            .service(
                web::scope("/auth")
                    .service(web::resource("/signup").route(web::post().to(users::signup)))
                    .service(web::resource("/login").route(web::post().to(users::login)))
                    .service(
                        web::resource("/admin")
                            .route(web::post().to(users::add_admin))
                            .wrap(auth.clone()),
                    ),
            )
            .service(
                web::scope("/posts")
                    .service(
                        web::resource("")
                            .route(web::get().to(posts::get_all))
                            .route(
                                web::post()
                                    .guard(guard::fn_guard(|head| {
                                        token_util::check_role(head, "ROLE_ADMIN")
                                    }))
                                    .to(posts::create_post),
                            ),
                    )
                    .service(
                        web::resource("/author")
                            .route(web::get().to(posts::get_by_author))
                            .wrap(auth.clone()),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(posts::get_by_id))
                            .route(
                                web::delete()
                                    .guard(guard::fn_guard(|head| {
                                        token_util::check_role(head, "ROLE_ADMIN")
                                    }))
                                    .to(posts::delete),
                            ),
                    ),
            )
            .service(
                web::scope("/comments")
                    .service(
                        web::resource("").route(
                            web::post()
                                .guard(guard::fn_guard(|head| {
                                    token_util::check_role(head, "ROLE_USER")
                                }))
                                .to(comments::create_comment),
                        ),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(comments::get_comments_for_post))
                            .route(
                                web::delete()
                                    .guard(guard::fn_guard(|head| {
                                        token_util::check_role(head, "ROLE_USER")
                                    }))
                                    .to(comments::delete_comment),
                            ),
                    ),
            )
            .service(
                web::scope("/rates")
                    .service(
                        web::resource("").route(
                            web::post()
                                .guard(guard::fn_guard(|head| {
                                    token_util::check_role(head, "ROLE_USER")
                                }))
                                .to(rates::rate_post),
                        ),
                    )
                    .service(
                        web::resource("/post/{id}").route(
                            web::get()
                                .guard(guard::fn_guard(|head| {
                                    token_util::check_role(head, "ROLE_USER")
                                }))
                                .to(rates::get_user_rate_for_post),
                        ),
                    ),
            ),
    );
}
