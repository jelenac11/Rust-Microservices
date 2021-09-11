use diesel;
use diesel::prelude::*;

use crate::model::NewPost;
use crate::model::Post;

use crate::schema::posts;
use crate::schema::posts::dsl::*;

pub fn create_post(new_post: NewPost, conn: &PgConnection) -> QueryResult<Post> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn get_all_posts(conn: &PgConnection) -> QueryResult<Vec<Post>> {
    posts.filter(published.eq(true)).limit(5).load::<Post>(conn)
}

pub fn get_post(post_id: i32, conn: &PgConnection) -> QueryResult<Post> {
    posts::table.find(post_id).get_result::<Post>(conn)
}

pub fn update_post(post_id: i32, post: Post, conn: &PgConnection) -> QueryResult<Post> {
    diesel::update(posts::table.find(post_id))
        .set(&post)
        .get_result(conn)
}

pub fn publish_post(post_id: i32, conn: &PgConnection) -> QueryResult<Post> {
    diesel::update(posts::table.find(post_id))
        .set(published.eq(true))
        .get_result(conn)
}

pub fn delete_post(post_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    diesel::delete(posts::table.find(post_id)).execute(conn)
}
