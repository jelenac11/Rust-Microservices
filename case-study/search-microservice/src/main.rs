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
use config::db;
use endpoints::{posts_controller, healthcheck};

mod schema;
mod model;
mod config;
mod endpoints;
mod services;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);

    let pool: db::Pool = db::init_pool();
    let log = config::log::config_logging();
    log4rs::init_config(log).expect("Configuring logging");
    

    HttpServer::new(move || {
        info!("Starting search microservice");
        App::new()
            .data(pool.clone())
            .service(posts_controller::search_post_by_title_and_description)
            .service(healthcheck::healthcheck)
    })
    .bind(&app_url)?
    .run()
    .await
}
