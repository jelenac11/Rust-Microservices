use crate::schema::{authors, posts};

#[derive(Insertable)]
#[table_name = "authors"]
pub struct NewAuthor {
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub description: String,
    pub author_id: i32
}