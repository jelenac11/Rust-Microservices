use diesel;
use diesel::prelude::*;

use crate::model::NewPost;
use crate::model::Post;

use crate::schema::posts;
use crate::schema::posts::dsl::*;
use crate::handlers::AverageRate;

pub fn create_post(new_post: NewPost, conn: &PgConnection) -> QueryResult<Post> {
    info!("Inserting new post in database");
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
}

pub fn get_all_posts(conn: &PgConnection) -> QueryResult<Vec<Post>> {
    info!("Getting all posts from database");
    posts.load::<Post>(conn)
}

pub fn get_post(post_id: i32, conn: &PgConnection) -> QueryResult<Post> {
    info!("Getting post from database by id");
    posts::table.find(post_id).get_result::<Post>(conn)
}

pub fn get_posts_by_author_id(uid: i32, conn: &PgConnection) -> QueryResult<Vec<Post>> {
    info!("Getting posts by author from database");
    posts.filter(author_id.eq(uid)).load::<Post>(conn)
}

pub fn delete_post(post_id: i32, conn: &PgConnection) -> QueryResult<usize> {
    info!("Deleting post from database");
    diesel::delete(posts::table.find(post_id)).execute(conn)
}

pub fn update_average_rate(average: AverageRate, conn: &PgConnection) -> QueryResult<Post> {
    info!("Updating average rate for specified post in database");
    diesel::update(posts.find(average.post_id))
        .set(rate.eq(average.average_rate as f64))
        .get_result(conn)
}
