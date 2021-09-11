use crate::{
    schema::rates::dsl::*,
    model::rate::{Rate, NewRate},
};
use diesel::prelude::*;

pub fn check_if_rate_for_post_exists(author: i32, post: i32, conn: &PgConnection) -> QueryResult<Vec<Rate>> {
    info!("Getting rate from database by logged user and post");
    rates.filter(post_id.eq(post).and(author_id.eq(author))).load::<Rate>(conn)
}

pub fn create_rate(rate: NewRate, conn: &PgConnection) -> QueryResult<Rate> {
    info!("Inserting rate for logged user in database");
    diesel::insert_into(rates)
        .values(&rate)
        .get_result(conn)
}

pub fn update_rate(rate: NewRate, rate_id: i32, conn: &PgConnection) -> QueryResult<Rate> {
    info!("Updating rate for logged user in database");
    diesel::update(rates.find(rate_id))
        .set(value.eq(rate.value))
        .get_result(conn)
}

pub fn get_rates_for_post(post: i32, conn: &PgConnection) -> QueryResult<Vec<Rate>> {
    info!("Getting all rates from database for specified post");
    rates.filter(post_id.eq(post)).load::<Rate>(conn)
}

pub fn get_user_rate_for_post(post: i32, user: i32, conn: &PgConnection) -> QueryResult<Vec<Rate>> {
    info!("Getting logged user rate from database for specified post");
    rates.filter(post_id.eq(post).and(author_id.eq(user))).load::<Rate>(conn)
}