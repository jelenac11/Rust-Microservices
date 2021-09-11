#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate juniper;
extern crate juniper_subscriptions;

use actix_web::{web, App, HttpServer};
use dotenv;
use handlers::configure_service;
use std::io;

mod graphql_schema;
mod handlers;
mod kafka;
mod repositories;
mod schema;
use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("Can't get DB URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let pool = create_connection_pool();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(configure_service)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("0.0.0.0:8080")
    .expect("Something went wrong")
    .run()
    .await
}
