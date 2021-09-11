#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
use actix_web::{App, HttpServer};

mod connection;
mod handlers;
mod model;
mod repository;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool: connection::Pool = connection::init_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(handlers::all_posts)
            .service(handlers::create_post)
            .service(handlers::delete_post)
            .service(handlers::get_post)
            .service(handlers::update_post)
            .service(handlers::publish_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
