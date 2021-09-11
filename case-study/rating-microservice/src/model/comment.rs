use validator::Validate;
use crate::schema::comments;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub text: String,
    pub post_id: i32,
    pub author_id: i32,
    pub author_username: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CommentDTO {
    #[validate(length(min = 1))]
    pub text: String,
    pub post_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "comments"]
pub struct NewComment {
    pub text: String,
    pub post_id: i32,
    pub author_id: i32,
    pub author_username: String,
}