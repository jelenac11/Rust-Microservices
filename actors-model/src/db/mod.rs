use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use std::env;

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
