use crate::{
    config::db::Pool,
    model::{
        rate::RateDTO
    },
    services::rates_service,
    utils::response_util,
};
use actix_web::{web, HttpRequest, error, Error, HttpResponse, Result};
use actix_web::web::Path;
use validator::Validate;

// POST api/rates
pub async fn rate_post(rate_dto: web::Json<RateDTO>, pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    trace!("Rating post requested");
    let rate = rate_dto.into_inner();
    rate.validate().map_err(|e| return error::ErrorBadRequest(e.to_string()))?;
    rates_service::rate_post(rate, &pool, req).await
        .map(|rate| HttpResponse::Ok().content_type("application/json").body(rate))
        .map_err(|error| error)
}

// GET api/rates/post/{id}
pub async fn get_user_rate_for_post(post_id: Path<i32>, pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
    trace!("Getting logged user rate for post requested");
    rates_service::get_user_rate_for_post(post_id.into_inner(), &pool, req)
        .map(|rate| {
            if rate.len() > 0 {
                return HttpResponse::Ok().json(rate.get(0));
            } else {
                return HttpResponse::NotFound().finish();
            }
        })
        .map_err(|error| response_util::error_response(error))
}