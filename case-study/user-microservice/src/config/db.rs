use std::env;
use diesel::pg::PgConnection;
use diesel::sql_query;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn init_pool() -> Pool {
    info!("Configuring database pool...");
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

#[cfg(not(test))]
fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}


#[cfg(test)]
fn database_url() -> String {
    env::var("DATABASE_TEST_URL").unwrap_or_else(|_| "postgres://postgres:zovemsejelenajelena@localhost/users-test".to_string())
}

#[cfg(test)]
pub fn migrate_and_config_db() -> Pool { 
    use crate::diesel::RunQueryDsl;

    let manager = ConnectionManager::<PgConnection>::new(database_url());
    let pool = Pool::new(manager).expect("db pool");

    sql_query(r#"DROP TABLE IF EXISTS users;"#).execute(&*pool.get().unwrap()).expect("Drop table users");
    sql_query(r#"CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR NOT NULL UNIQUE,
        email VARCHAR NOT NULL UNIQUE,
        firstName VARCHAR NOT NULL,
        lastName VARCHAR NOT NULL,
        password VARCHAR NOT NULL,
        role VARCHAR NOT NULL
    );"#).execute(&*pool.get().unwrap()).expect("Create table users");
    sql_query(r#"DELETE FROM users;"#).execute(&*pool.get().unwrap()).expect("Delete records from users");
    pool
}
