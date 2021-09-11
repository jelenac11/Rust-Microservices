use crate::{
    config::db::Pool,
    model::rate::{RateDTO, Rate, NewRate},
    utils::{token_util, response_util},
    repository::rates_repository,
    constants,
};
use actix_web::{web, web::Bytes, HttpRequest, error, Error as ActixError, http::StatusCode};
use awc;
use diesel::result::Error;

pub async fn rate_post(rate_dto: RateDTO, pool: &web::Data<Pool>, req: HttpRequest) -> Result<Bytes, ActixError> {
    info!("Rating post");
    let uid = token_util::get_user_id(req.clone()).unwrap();
    let client = awc::Client::new();
    let url = format!("{}{}{}", &constants::POST_MS, "/api/posts/", rate_dto.post_id);
    let resp = client.get(url)
        .send()
        .await;
    match resp {
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    //let post = response.json::<Post>().await;
                    info!("Post that is rate for exists");
                    let rate = NewRate {
                        value: rate_dto.value,
                        post_id: rate_dto.post_id,
                        author_id: uid
                    };
                    let connection = pool.get().expect("Connection from pool");
                    let post_rate = check_if_rate_for_post_exists(&rate, pool);
                    if let Some(existed_rate) = post_rate {
                        let avg = update_average_rate(&existed_rate, true, pool, &rate).await;
                        match avg {
                            Ok(avg) => {
                                rates_repository::update_rate(rate, existed_rate.id, &connection)
                                .map(|rate| rate)
                                .map_err(|error| response_util::error_response(error));
                                return Ok(avg);
                            },
                            Err(error) => {
                                return Err(error::ErrorInternalServerError(error.to_string()));
                            }
                        }
                    }
                    let avg = update_average_rate(&Rate::new(), false, pool, &rate).await;
                    match avg {
                        Ok(avg) => {
                            rates_repository::create_rate(rate, &connection)
                            .map(|rate| rate)
                            .map_err(|error| response_util::error_response(error));
                            return Ok(avg);
                        },
                        Err(error) => {
                            return Err(error::ErrorInternalServerError(error.to_string()));
                        }
                    }
                }
                StatusCode::NOT_FOUND => {
                    info!("Post that is rate for doesn't exist");
                    return Err(error::ErrorNotFound("Post doesn't exist!"));
                }
                _ => {
                    debug!("Some error occured while rating the post");
                    return Err(error::ErrorInternalServerError("Error occured!"));
                }
            }
        }
        Err(error) => {
            debug!("{}", format!("Error occured: {:?}", error));
            return Err(error::ErrorInternalServerError(error.to_string()));
        }
    }
}

pub fn get_user_rate_for_post(id: i32, pool: &web::Data<Pool>, req: HttpRequest) -> Result<Vec<Rate>, Error> {
    info!("Getting logger user rate for specified post");
    let connection = pool.get().expect("Connection from pool");
    let uid = token_util::get_user_id(req.clone()).unwrap();
    rates_repository::get_user_rate_for_post(id, uid, &connection)
}

fn check_if_rate_for_post_exists(rate: &NewRate, pool: &web::Data<Pool>) -> Option<Rate> {
    info!("Checking have logged user already rated post");
    let connection = pool.get().expect("Connection from pool");
    let rate = rates_repository::check_if_rate_for_post_exists(rate.author_id.clone(), rate.post_id.clone(), &connection).unwrap();
    if rate.len() > 0 {
        return Some(rate[0].to_owned());
    }
    None
}

async fn update_average_rate(rate: &Rate, exists: bool, pool: &web::Data<Pool>, new: &NewRate) -> Result<Bytes, ActixError> {
    info!("Calculating average rate for post");
    let connection = pool.get().expect("Connection from pool");
    let all_rates_for_post = rates_repository::get_rates_for_post(rate.post_id, &connection);
    if exists {
        let rates = all_rates_for_post.unwrap();
        let number = rates.len();
        let mut sum: i32 = rates.iter().map(|p| p.value).sum();
        sum -= rate.value;
        sum += new.value;
        return update_average_rate_request((sum as f32)/(number as f32), new.post_id).await;
    } else {
        match all_rates_for_post {
            Ok(rates) => {
                let number = rates.len() + 1;
                let mut sum: i32 = rates.iter().map(|p| p.value).sum();
                sum += new.value;
                return update_average_rate_request((sum as f32)/(number as f32), new.post_id).await;
            },
            Err(error) => {
                return Err(error::ErrorInternalServerError(error.to_string()));
            }
        }
    }
}

async fn update_average_rate_request(sum: f32, post: i32) -> Result<Bytes, ActixError> {
    info!("Calling post microservise to update average rate for post");
    let client = awc::Client::new();
    let url = format!("{}{}{}{}{}", &constants::POST_MS, "/api/posts?post_id=", post, "&average_rate=", sum);
    let resp = client.put(url)
        .send()
        .await;
    match resp {
        Ok(mut response) => {
            let body = response.body()
                    .limit(20_000_000)
                    .await
                    .unwrap();
            info!("Successfully updated average rate of post");
            return Ok(body);
        }
        Err(error) => {
            debug!("{}", format!("Error occured: {:?}", error));
            return Err(error::ErrorInternalServerError(error.to_string()));
        }
    }
}