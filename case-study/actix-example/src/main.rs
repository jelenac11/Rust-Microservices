#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::{App, HttpServer};
use std::env;

mod connection;
mod handlers;
mod model;
mod repository;
mod schema;
mod log_config;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);

    let pool: connection::Pool = connection::init_pool();
    let log = log_config::config_logging();
    log4rs::init_config(log).expect("Configuring logging");

    HttpServer::new(move || {
        info!("Starting posts microservice");
        App::new()
            .data(pool.clone())
            .service(handlers::get_posts_by_author_id)
            .service(handlers::get_all_posts)
            .service(handlers::create_post)
            .service(handlers::delete_post)
            .service(handlers::get_post)
            .service(handlers::update_average_rate)
    })
    .bind(&app_url)?
    .run()
    .await
}
