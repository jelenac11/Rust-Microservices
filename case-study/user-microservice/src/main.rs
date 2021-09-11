#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix_rt;
extern crate dotenv;
extern crate r2d2;
extern crate bcrypt;
extern crate derive_more;
//extern crate lettre;

use actix_web::{http, App, HttpServer};
use std::env;
use actix_cors::Cors;

mod config;
mod endpoints;
mod models;
mod errors;
mod services;
mod schema;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);   

    let pool = config::db::init_pool();
    let log = config::log::config_logging();
    log4rs::init_config(log).expect("Configuring logging");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                //.allowed_origin(app_url.clone())
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .data(pool.clone())
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .run()
    .await
}
