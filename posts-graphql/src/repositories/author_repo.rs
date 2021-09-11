use diesel::prelude::*;

use crate::repositories::input::NewAuthor;
use crate::schema::{authors, posts};
use crate::graphql_schema::{Author, Post, AuthorInput};

pub fn get_all_authors(conn: &PgConnection) -> Vec<Author> {
    use crate::schema::authors::dsl::*;

    authors
      .limit(100)
      .load::<Author>(conn)
      .expect("Error loading authors")
}

pub fn get_author(id: i32, conn: &PgConnection) -> QueryResult<Author> {
    authors::table.find(id).get_result::<Author>(conn)
}

pub fn get_author_posts(id: i32, conn: &PgConnection) -> Vec<Post> {
    posts::table
        .filter(posts::author_id.eq(id))
        .load::<Post>(conn)
        .expect("Error loading author posts")
}

pub fn create_author(new: AuthorInput, conn: &PgConnection) -> QueryResult<Author> {
    use crate::schema::authors::dsl::*;

    diesel::insert_into(authors)
        .values(NewAuthor { name: new.name.clone()})
        .get_result(conn)
}
