use crate::{model::post::Post, schema::posts::dsl::*};
use diesel::prelude::*;

pub fn get_by_title_and_description(key_word: &str, conn: &PgConnection) -> QueryResult<Vec<Post>> {
    info!(
        "{}",
        format!("Reading posts from database, key word {}", key_word)
    );
    let pattern = format!("{}{}{}", "%", key_word, "%");
    posts
        .filter(title.ilike(&pattern))
        .or_filter(text.ilike(&pattern))
        .load::<Post>(conn)
}
