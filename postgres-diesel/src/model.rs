use serde_derive::Serialize;
use super::schema::posts;

#[derive(Debug, Serialize, Queryable, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub description: &'a str,
}