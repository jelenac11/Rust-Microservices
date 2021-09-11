use crate::schema::posts;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct PostEntity {
    pub id: i32,
    pub title: String,
    pub text: String
}