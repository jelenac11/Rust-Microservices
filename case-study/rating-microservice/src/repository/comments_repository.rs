use crate::{
    schema::comments::dsl::*,
    model::comment::{Comment, NewComment},
};
use diesel::prelude::*;

pub fn create_comment(comment: NewComment, conn: &PgConnection) -> QueryResult<Comment> {
    info!("{}", format!("Inserting comment of user with username {}", &comment.author_username));
    diesel::insert_into(comments)
        .values(&comment)
        .get_result(conn)
}

pub fn get_comment(comment_id: i32, conn: &PgConnection) -> QueryResult<Comment> {
    info!("Getting comment from database by id of comment");
    comments.find(comment_id).get_result::<Comment>(conn)
}

pub fn delete_comment(comment_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("Deleting comment from database");
    diesel::delete(comments.find(comment_id)).execute(conn)
}

pub fn get_comments_for_post(id_post: i32, conn: &PgConnection) -> QueryResult<Vec<Comment>> {
    info!("Getting all comments from database for requested post");
    comments.filter(post_id.eq(id_post)).load::<Comment>(conn)
}