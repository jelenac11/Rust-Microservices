#[macro_use]
extern crate actix_web;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use std::env;
use utils::token_util;

mod config;
mod constants;
mod endpoints;
mod errors;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .configure(config::app::config_services)
    })
    .bind(&app_url)?
    .run()
    .await
}
