use crate::config::db::Pool;
use crate::services::posts_service;
use actix_web::web::{Data};
use actix_web::web;
use actix_web::{Error, HttpResponse};
use crate::utils::response_util;

#[derive(Deserialize)]
pub struct Search {
    search: String,
}

#[get("/api/posts")]
pub async fn search_post_by_title_and_description(web::Query(key_word): web::Query<Search>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    trace!("{}", format!("Search posts requested with key word {}", &key_word.search));
    posts_service::search_post_by_title_and_description(key_word.search.to_lowercase(), &pool)
        .map(|posts| HttpResponse::Ok().json(posts))
        .map_err(|error| response_util::error_response(error))
}