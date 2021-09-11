use crate::config::db::Pool;
use crate::model::post::Post;
use crate::repository::posts_repository;
use actix_web::web::Data;
use diesel::result::Error;

pub fn search_post_by_title_and_description(
    key_word: String,
    pool: &Data<Pool>,
) -> Result<Vec<Post>, Error> {
    let connection = pool.get().expect("Connection from pool");

    match posts_repository::get_by_title_and_description(&key_word, &connection) {
        Ok(posts) => {
            info!(
                "{}",
                format!(
                    "Searching posts with keyword {} finished successfully",
                    &key_word
                )
            );
            Ok(posts)
        }
        Err(error) => {
            debug!(
                "{}",
                format!("Searching posts with key word {} failed", &key_word)
            );
            Err(error)
        }
    }
}
