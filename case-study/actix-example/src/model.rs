use crate::schema::posts;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub rate: f64,
    pub author_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub text: String,
    pub rate: f64,
    pub author_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct PostDTO {
    pub title: String,
    pub text: String,
}
