#[derive(Serialize, Deserialize)]
pub struct CommentDTO {
    pub text: String,
    pub post_id: i32,
}