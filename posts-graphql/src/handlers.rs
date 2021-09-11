use actix_web::{web, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
use crate::PgPool;
use crate::graphql_schema::{create_schema, Schema, Context, get_loader};

pub fn configure_service(config: &mut web::ServiceConfig) {
    config
        .data(create_schema())
        .route("/graphql", web::post().to(graphql))
        .route("/graphiql", web::get().to(graphiql));
}

pub async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let ctx = Context {
        db: Arc::new(pool.get_ref().to_owned()),
        loader: get_loader()
    };
    let res = data.execute(&schema, &ctx).await;

    HttpResponse::Ok().json(res)
}

pub async fn graphiql() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/graphql", None))
}