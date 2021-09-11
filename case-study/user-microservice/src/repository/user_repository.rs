use crate::{
    schema::users::dsl::*,
    models::user::{UserDTO, User},
};
use diesel::prelude::*;


pub fn signup(user: UserDTO, conn: &PgConnection) -> QueryResult<usize> {
    info!("Adding new user to database");
    diesel::insert_into(users).values(&user).execute(conn)
}

pub fn find_user_by_email(mail: &str, conn: &PgConnection) -> QueryResult<User> {
    info!("Getting user by email from database");
    users.filter(email.eq(mail)).get_result::<User>(conn)
}