use diesel::prelude::*;

use crate::schema::posts;
use crate::model::{Post, NewPost};

pub fn get_all_posts(conn: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    posts
      .limit(100)
      .load::<Post>(conn)
      .expect("Error loading posts")
}

pub fn get_post(id: i32, conn: &PgConnection) -> QueryResult<Post> {
    posts::table.find(id).get_result::<Post>(conn)
}

pub fn create_post(new: NewPost, conn: &PgConnection) -> QueryResult<Post> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts)
        .values(new)
        .get_result(conn)
}

pub fn delete_post(desc: String, conn: &PgConnection) -> usize{
    use crate::schema::posts::dsl::*;

    diesel::delete(posts.filter(description.like(desc)))
        .execute(conn)
        .expect("Error deleting posts")
}

pub fn update_post(id: i32, desc: String, conn: &PgConnection) -> Post {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(id))
        .set(description.eq(desc))
        .get_result::<Post>(conn)
        .expect("Post not found")
}
