#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate actix;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
use crate::actix::Actor;
use actix::SyncArbiter;
use actix_web::{App, HttpServer};
use actors::count::CountActor;
use actors::db::DbActor;
use handlers::handlers::*;
use model::model::AppState;

mod actors;
mod db;
mod handlers;
mod model;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = db::get_pool();
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    let count_addr = CountActor::new().start();

    let state = AppState {
        db: db_addr,
        count: count_addr,
    };
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .service(all_posts)
            .service(create_post)
            .service(delete_post)
            .service(get_post)
            .service(update_post)
            .service(publish_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
