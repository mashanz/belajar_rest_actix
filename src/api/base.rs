use crate::api::error_response::method_not_allowed;
use actix_web::{web, HttpResponse, Resource, Responder};
use serde_json::json;

/// Get Method Rest API
async fn get() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Cuma Bisa Get",
    }))
}

/// Register Method Rest API (`GET`, `POST`, `PUT`, `DELETE`) method di dalam 1 resource path `/`
pub fn services() -> Resource {
    web::resource("/")
        .route(web::get().to(get))
        .default_service(web::route().to(method_not_allowed))
}
