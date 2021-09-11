use diesel::prelude::*;

use crate::model::PostEntity;
use crate::schema::posts;
use crate::Conn;

pub fn get_titles(conn: &Conn) -> QueryResult<Vec<String>> {
    let names = posts::table.select(posts::title).load(conn)?;
    Ok(names)
}

pub fn get_all(conn: &Conn) -> QueryResult<Vec<PostEntity>> {
    let posts: Vec<PostEntity> = posts::table.load::<PostEntity>(conn)?;

    Ok(posts)
}

pub fn get_by_id(id: u64, conn: &Conn) -> QueryResult<PostEntity> {
    let post: PostEntity = posts::table
        .filter(posts::id.eq(id as i32))
        .first(conn)?;
    Ok(post)
}
