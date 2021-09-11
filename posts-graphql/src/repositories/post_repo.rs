use diesel::prelude::*;

use crate::schema::posts;
use crate::repositories::input::{NewPost};
use crate::graphql_schema::{Post, PostInput};

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

pub fn create_post(new: PostInput, conn: &PgConnection) -> QueryResult<Post> {
    use crate::schema::posts::dsl::*;

    diesel::insert_into(posts)
        .values(NewPost { title: new.title.clone(), description: new.description.clone(), author_id: new.author_id})
        .get_result(conn)
}