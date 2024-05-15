use crate::api::error_response::method_not_allowed;
use actix_web::{web, HttpResponse, Resource, Responder};
use serde_json::json;
use crate::app_data::AppData;

/// Get Method Rest API
async fn get(
    data: actix_web::web::Data<AppData>
) -> impl Responder {
    let mut context = tera::Context::new();
    context.insert("NAMA", "qwerty");
    let rendered_page = data.template.render("index.html", &context).unwrap();

    // HttpResponse::Ok().json(json!({
    //     "message": "Cuma Bisa Get Aja!",
    // }))

    actix_web::HttpResponse::Ok().body(rendered_page)
}

/// Register Method Rest API (`GET`, `POST`, `PUT`, `DELETE`) method di dalam 1 resource path `/`
pub fn services() -> Resource {
    web::resource("/")
        .route(web::get().to(get))
        .default_service(web::route().to(method_not_allowed))
}
