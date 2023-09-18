use crate::api::error_response::method_not_allowed;
use actix_web::{web, HttpResponse, Resource, Responder};
use serde_json::json;

#[derive(serde::Deserialize)]
struct QueryParams {
    key_1: String,
    key_2: Option<String>,
}

/// Get Method Rest API
async fn get(query_params: web::Query<QueryParams>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "key_1": query_params.key_1.as_str(),
        "key_2": query_params.key_2.as_deref().unwrap_or(""),
    }))
}

/// Register Method Rest API (`GET`, `POST`, `PUT`, `DELETE`) method di dalam 1 resource path `/`
pub fn services() -> Resource {
    web::resource("/coba_query_param")
        .route(web::get().to(get))
        .default_service(web::route().to(method_not_allowed))
}
