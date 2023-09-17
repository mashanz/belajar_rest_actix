use actix_web::{web, HttpResponse, Resource, Responder};
use serde_json::json;

/// Get Method Rest API
async fn get() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Ini Get",
    }))
}

/// Post Method Rest API
async fn post() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Ini Post",
    }))
}

/// Put Method Rest API
async fn put() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Ini Put",
    }))
}

/// Delete Method Rest API
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Ini Delete",
    }))
}

/// Register Method Rest API (`GET`, `POST`, `PUT`, `DELETE`) method di dalam 1 resource path `/`
pub fn services() -> Resource {
    web::resource("/")
        .route(web::get().to(get))
        .route(web::post().to(post))
        .route(web::put().to(put))
        .route(web::delete().to(delete))
}
