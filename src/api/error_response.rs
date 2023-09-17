use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde_json::json;

/// Method Not Allowed Rest API
pub async fn method_not_allowed() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .json(json!({
            "message": "Method Not Allowed",
        }))
}

/// Not Found Rest API
pub async fn not_found() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::NOT_FOUND)
        .json(json!({
            "message": "Not Found",
        }))
}
