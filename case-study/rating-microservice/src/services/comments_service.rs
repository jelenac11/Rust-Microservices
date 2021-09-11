use crate::{
    config::db::Pool,
    model::comment::{CommentDTO, Comment, NewComment},
    utils::{token_util, response_util},
    repository::comments_repository,
};
use actix_web::{web, HttpRequest, error, Error as ActixError};
use diesel::result::Error;

pub fn create_comment(comment_dto: CommentDTO, pool: &web::Data<Pool>, req: HttpRequest) -> Result<Comment, Error> {
    let uid = token_util::get_user_id(req.clone()).unwrap();
    let username = token_util::get_user_username(req).unwrap();
    let comment = NewComment {
        text: comment_dto.text,
        post_id: comment_dto.post_id,
        author_id: uid,
        author_username: username
    };
    let connection = pool.get().expect("Connection from pool");

    comments_repository::create_comment(comment, &connection)
}

pub fn delete_comment(id: i32, pool: &web::Data<Pool>, req: HttpRequest) -> Result<String, ActixError> {
    let connection = pool.get().expect("Connection from pool");
    let uid = token_util::get_user_id(req).unwrap();
    let post = comments_repository::get_comment(id, &connection);
    if let Ok(existed_post) = post {
        if existed_post.author_id == uid {
            return comments_repository::delete_comment(id, &connection)
                        .map(|_| "Comment successfully deleted!".to_string())
                        .map_err(|error| response_util::error_response(error));
        } else {
            return Err(error::ErrorBadRequest("Only author of comment can delete comment!".to_string()));
        }
    }
    Err(error::ErrorNotFound(format!("Comment with id {} not found!", id)))
}

pub fn get_comments_for_post(id: i32, pool: &web::Data<Pool>) -> Result<Vec<Comment>, Error> {
    let connection = pool.get().expect("Connection from pool");

    comments_repository::get_comments_for_post(id, &connection)
}