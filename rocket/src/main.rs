#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

mod connection;
mod handlers;
mod model;
mod repository;
mod schema;

fn main() {
    dotenv::dotenv().ok();

    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/posts",
            routes![
                handlers::create_post,
                handlers::all_posts,
                handlers::get_post,
                handlers::update_post,
                handlers::delete_post,
                handlers::publish_post
            ],
        )
        .launch();
}
