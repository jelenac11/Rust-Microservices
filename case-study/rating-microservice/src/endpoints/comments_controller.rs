use crate::{
    config::db::Pool,
    model::{
        comment::CommentDTO
    },
    services::comments_service,
    utils::response_util,
};
use actix_web::{web, HttpRequest, error, Error, HttpResponse, Result};
use actix_web::web::{Data, Path};
use validator::Validate;

// POST api/comments
pub async fn create_comment(post_dto: web::Json<CommentDTO>, pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    trace!("Creating comment requested");
    let post = post_dto.into_inner();
    post.validate().map_err(|e| return error::ErrorBadRequest(e.to_string()))?;
    comments_service::create_comment(post, &pool, req)
        .map(|comment| response_util::comment_created(comment))
        .map_err(|error| response_util::error_response(error))
}

// DELETE api/comments/{id}
pub async fn delete_comment(id: Path<i32>, pool: Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    trace!("Deleting comment requested");
    comments_service::delete_comment(id.into_inner(), &pool, req)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(|error| error)
}

// GET api/comments/{id}
pub async fn get_comments_for_post(id: Path<i32>, pool: Data<Pool>) -> Result<HttpResponse, Error> {
    trace!("{}", format!("Getting comments for post {} requested", id));
    comments_service::get_comments_for_post(id.into_inner(), &pool)
        .map(|comments| HttpResponse::Ok().json(comments))
        .map_err(|error| response_util::error_response(error))
}