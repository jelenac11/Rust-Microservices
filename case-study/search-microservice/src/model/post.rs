use crate::schema::posts;

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub rate: f64,
    pub author_id: i32,
}